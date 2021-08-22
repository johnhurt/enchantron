use super::{
    Entity, Gor, LocationKey, LocationWriteResponse, Time, TimeSource,
};
use crate::model::{IPoint, IRect, ISize, Point, Rect};
use one_way_slot_map::SlotMap;
use rstar::{PointDistance, RTree, RTreeObject};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;
use tokio::sync::RwLock;

// This value determines the window size for an entity based on its max speed
const MAX_SECS_BETWEEN_REINSERTS: f64 = 8.;

// This value determines how close to the edge of the window an entity is
// allowed to get before the window is moved
const MIN_SECS_BETWEEN_REINSERTS: f64 = 0.5;

// Movement operations on the location service will provide a time at which the
// entity will need to be updated. This time will be when the entity is
// calculated to exit the window multiplied by this safety factor
const UPDATE_INTERVAL_SAFETY_FRACTION: f64 = 0.75;

// This is the distance limit at which two points are considered coincident
// if they are closer than this distance (for the purposes of calculating
// offsets)
const MINIMUM_DISTANCE: f64 = 0.001;

#[derive(Debug, Copy, Clone, derive_new::new)]
pub struct Movement {
    pub destination: Point,
    pub velocity: Point,
    pub max_valid_time: f64
}

// Default
#[derive(Debug, Copy, Clone, derive_new::new)]
pub struct WindowedLocation {
    pub ref_center: Point,
    pub radius: f64,
    pub ref_time: f64,
    pub max_speed: f64,
    pub movement: Option<Movement>,
    pub entity: Entity,
    pub window: IRect,
}

#[derive(Debug)]
struct WindowedLocationPointer {
    owner: bool,
    p: *const WindowedLocation,
    _marker: PhantomData<WindowedLocation>,
}

impl PartialEq for WindowedLocationPointer {
    fn eq(&self, other: &WindowedLocationPointer) -> bool {
        self.entity == other.entity
    }
}

impl RTreeObject for WindowedLocationPointer {
    type Envelope = IRect;

    fn envelope(&self) -> Self::Envelope {
        self.window
    }
}

impl PointDistance for WindowedLocationPointer {
    /// Returns the squared euclidean distance of an object to a point.
    fn distance_2(&self, point: &IPoint) -> i64 {
        self.window.distance_squared(point)
    }

    /// Returns true if a point is contained within this object.
    fn contains_point(&self, point: &IPoint) -> bool {
        self.window.contains_point(point)
    }

    /// Returns the squared distance to this object or `None` if the distance
    /// is larger than a given maximum value.
    fn distance_2_if_less_or_equal(
        &self,
        point: &IPoint,
        max_distance_2: i64,
    ) -> Option<i64> {
        let distance_2 = self.distance_2(point);

        if distance_2 <= max_distance_2 {
            Some(distance_2)
        } else {
            None
        }
    }
}

unsafe impl Send for WindowedLocationPointer {}
unsafe impl Sync for WindowedLocationPointer {}

impl Drop for WindowedLocationPointer {
    fn drop(&mut self) {
        if self.owner {
            unsafe {
                ptr::drop_in_place(self.p as *mut WindowedLocation);
            }
        }
    }
}

impl PartialEq for WindowedLocation {
    fn eq(&self, other: &WindowedLocation) -> bool {
        self.entity == other.entity
    }
}

impl From<WindowedLocation> for WindowedLocationPointer {
    fn from(src: WindowedLocation) -> Self {
        WindowedLocationPointer {
            owner: true,
            p: Box::into_raw(Box::new(src)),
            _marker: Default::default(),
        }
    }
}

impl WindowedLocation {
    /// Check to see if this window contains the center it represents
    fn check_window_contains_location(&self) -> bool {
        self.window.contains_point(&self.ref_center.floor())
    }

    /// Move the window for this windowed pointer to be the rectangle around the
    /// actual location
    fn recenter_window(&mut self) {
        let dist = self.radius + MAX_SECS_BETWEEN_REINSERTS * self.max_speed;

        self.window = IRect {
            top_left: self.ref_center.floor(),
            size: ISize::new(1, 1),
        };

        self.window = self.window.expanded_by(dist.ceil() as usize);
    }

    /// Get the topmost and leftmost point that the center can be inside the
    /// window.
    fn center_limit_rect(&self) -> Rect {
        let mut result = self.window.as_rect();
        result.top_left.x += self.radius;
        result.top_left.y += self.radius;
        result.size.width -= 2. * self.radius;
        result.size.height -= 2. * self.radius;
        result
    }

    ///  Get the time until any part of the entity exits the window
    fn time_until_boundary(&self) -> Option<f64> {

        self.movement.map(|movement| {

            let center_limit_rect = self.center_limit_rect();
            let offset_from_limit = self.ref_center - center_limit_rect.top_left;

            let ref velocity = movement.velocity;

            let x_time = if velocity.x < 0. {
                offset_from_limit.x / velocity.x
            } else if velocity.x > 0. {
                (center_limit_rect.size.width - offset_from_limit.x)
                    / velocity.x
            } else {
                f64::MAX
            };

            let y_time = if velocity.y < 0. {
                offset_from_limit.y / velocity.y
            } else if velocity.y > 0. {
                (center_limit_rect.size.width - offset_from_limit.y)
                    / velocity.y
            } else {
                f64::MAX
            };

            x_time.min(y_time)
        })

    }

    /// Since the location is stored at a specific time and with a fixed
    /// velocity, we need to know the time we are measuring the position at
    fn get_center_at_time(&self, time: f64) -> Point {
        self.ref_center + self.movement.map(|movement| {
            let sample_time = time.min(movement.max_valid_time);
            movement.velocity * (sample_time - self.ref_time)
        }).unwrap_or_default()
    }

    /// Use the center by using dead reckoning from the current center and ref
    /// time to the given current time at the current velocity
    fn update_center(&mut self, new_ref_time: f64) {
        let new_center =
            self.ref_center + self.velocity * (new_ref_time - self.ref_time);
        self.ref_center = new_center;
        self.ref_time = new_ref_time;

        let time_to_boundary = self.time_until_boundary();
        self.max_valid_time = new_ref_time + time_to_boundary;
    }

    /// Check to see if the entity plus radius contains the given point at the
    /// given time
    fn check_entity_contains(&self, point: &Point, time: f64) -> bool {
        self.get_center_at_time(time).distance_squared_to(point) <= self.radius * self.radius
    }
}

impl WindowedLocationPointer {
    fn new(
        entity: Entity,
        ref_center: Point,
        velocity: Point,
        radius: f64,
        max_speed: f64,
        ref_time: f64,
        max_valid_time: f64,
        window: IRect,
    ) -> WindowedLocationPointer {
        WindowedLocation::new(
            ref_center, max_valid_time, radius, ref_time, max_speed, velocity, entity, window,
        )
        .into()
    }

    fn new_with_centered_window(
        entity: Entity,
        ref_center: Point,
        velocity: Point,
        radius: f64,
        max_speed: f64,
        time: &impl TimeSource,
    ) -> WindowedLocationPointer {
        let mut result = WindowedLocationPointer::new(
            entity,
            ref_center,
            velocity,
            radius,
            max_speed,
            time.current_time(),
            f64::MAX,
            IRect::default(),
        );

        result.recenter_window();

        result
    }
}

impl Deref for WindowedLocationPointer {
    type Target = WindowedLocation;

    fn deref(&self) -> &Self::Target {
        // Safety: This is safe as long as WindowLocationPointers are only ever
        // read from/written to behind a mutex. We can guarantee this is always
        // true because WindowedLocationPointers are private to this module, and
        // all accesses into the module's service are done through a RwMutex
        unsafe { &*self.p }
    }
}

impl DerefMut for WindowedLocationPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: This is safe as long as WindowLocationPointers are only ever
        // read from/written to behind a mutex. We can guarantee this is always
        // true because WindowedLocationPointers are private to this module, and
        // all accesses into the module's service are done through a RwMutex
        unsafe { &mut *(self.p as *mut WindowedLocation) }
    }
}

impl Clone for WindowedLocationPointer {
    fn clone(&self) -> WindowedLocationPointer {
        WindowedLocationPointer {
            owner: false,
            p: self.p,
            _marker: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocationService {
    inner: Gor<RwLock<LocationServiceInner<Time>>>,
}

#[derive(Debug)]
struct LocationServiceInner<TS>
where
    TS: TimeSource,
{
    time: TS,
    rtree: RTree<WindowedLocationPointer>,
    slot_map: SlotMap<LocationKey, Entity, WindowedLocationPointer>,
    slot_keys_by_entity: HashMap<Entity, LocationKey>,
}

macro_rules! with_inner {
    ($(
        $(#[$outer:meta])*
        $v:vis fn $fn_name:ident(&$inner:ident $(, $arg:ident : $arg_type:ty )* ) -> $ret_type:ty $body:block
    )+) => {
        $(
            impl <TS: TimeSource> LocationServiceInner<TS> {
                $(#[$outer])*
                fn $fn_name($inner : &LocationServiceInner<TS>, $($arg: $arg_type),*) -> $ret_type $body
            }

            impl LocationService {
                $v async fn $fn_name(&self, $($arg: $arg_type),*) -> $ret_type {
                    let inner = self.inner.read().await;
                    LocationServiceInner::$fn_name(&*inner, $($arg),*)
                }
            }
        )+
    };
}

macro_rules! with_inner_mut {
    ($(
        $(#[$outer:meta])*
        $v:vis fn $fn_name:ident(&mut $inner:ident $(, $arg:ident : $arg_type:ty )* ) -> $ret_type:ty $body:block
    )+) => {
        $(
            impl <TS: TimeSource> LocationServiceInner<TS> {
                $(#[$outer])*
                fn $fn_name($inner : &mut LocationServiceInner<TS>, $($arg: $arg_type),*) -> $ret_type $body
            }

            impl LocationService {
                $v async fn $fn_name(&self, $($arg: $arg_type),*) -> $ret_type {
                    let mut inner = self.inner.write().await;
                    LocationServiceInner::$fn_name(&mut *inner, $($arg),*)
                }
            }
        )+
    };
}

#[allow(dead_code)]
impl LocationService {
    pub fn new(time: Time) -> (LocationService, impl FnOnce() + Send) {
        let boxed_inner =
            Box::new(RwLock::new(LocationServiceInner::new(time)));
        let inner = Gor::new(&boxed_inner);

        (LocationService { inner }, move || drop(boxed_inner))
    }

    pub fn new_from_data(
        time: Time,
        data: &SlotMap<LocationKey, Entity, WindowedLocation>,
    ) -> (LocationService, impl FnOnce() + Send) {
        let boxed_inner = Box::new(RwLock::new(
            LocationServiceInner::new_from_data(time, data),
        ));
        let inner = Gor::new(&boxed_inner);

        (LocationService { inner }, move || drop(boxed_inner))
    }
}

#[allow(dead_code)]
impl<TS: TimeSource> LocationServiceInner<TS> {
    fn new(time: TS) -> Self {
        LocationServiceInner {
            time,
            rtree: RTree::new(),
            slot_map: SlotMap::new(),
            slot_keys_by_entity: Default::default(),
        }
    }

    fn new_from_data(
        time: TS,
        data: &SlotMap<LocationKey, Entity, WindowedLocation>,
    ) -> Self {
        let mut rtree = RTree::new();
        let mut slot_keys_by_entity = HashMap::default();

        data.iter(|loc| loc.entity).for_each(|(location_key, loc)| {
            slot_keys_by_entity.insert(loc.entity, location_key);
        });

        let slot_map = data.map(|loc| {
            let wp = WindowedLocationPointer::from(loc.clone());
            let wp_clone = wp.clone();

            rtree.insert(wp);
            wp_clone
        });

        LocationServiceInner {
            time,
            rtree,
            slot_map,
            slot_keys_by_entity,
        }
    }

    fn recenter_window_if_needed(&mut self, wp: &mut WindowedLocationPointer, now: f64) -> LocationWriteResponse {

        let mut time_to_boundary = wp.time_until_boundary();

        // We need to recenter the window if the center is too close to the
        // boundary or it has escaped all together
        if time_to_boundary < MIN_SECS_BETWEEN_REINSERTS
            || !wp.check_window_contains_location()
        {
            let mut owned_wp = self
                .rtree
                .remove(wp)
                .expect("Entity pointer should still be in rtree");
            owned_wp.recenter_window();

            time_to_boundary = owned_wp.time_until_boundary();

            self.rtree.insert(owned_wp);
        }

        let next_update_time = now
            + wp.time_until_boundary() * UPDATE_INTERVAL_SAFETY_FRACTION;

        LocationWriteResponse::new(next_update_time, None)
    }
}

// Define the functions that only require read access to the location service
with_inner! {

    /// Get the entities that contain the given point
    pub fn get_entities_at(&self, point: &Point) -> Vec<Entity> {
        self.rtree
            .locate_all_at_point(&point.floor())
            .filter(|wp| {
                wp.check_entity_contains(point, self.time.current_time())
            })
            .map(|wp| wp.entity)
            .collect()
    }

    /// Get the location center of the entity with the given key
    pub fn get_by_key(&self, key: &LocationKey) -> Option<Point> {
        self.slot_map.get(key).map(|wp| {
            wp.get_center_at_time(self.time.current_time())
        })
    }

    /// Get the location and location key of the given entity
    pub fn get_by_entity(&self, entity: &Entity) -> Option<(LocationKey, Point)> {
        if let Some(key) = self.slot_keys_by_entity.get(entity) {
            Some((
                *key,
                self.get_by_key(key).expect(
                    "If the key is present, the location should be too",
                ),
            ))
        } else {
            None
        }
    }
}

// Define the functions that require write access to the location service
with_inner_mut! {

    /// Insert the entity with the given initial conditions
    pub fn insert(
        &mut self,
        center: Point,
        radius: f64,
        max_speed: f64,
        velocity: Point,
        entity: Entity
    ) -> (LocationKey, LocationWriteResponse) {
        let wp = WindowedLocationPointer::new_with_centered_window(
            entity, center, velocity, radius, max_speed, &self.time,
        );
        let wp_clone = wp.clone();

        let next_update_time = self.time.current_time()
            + wp.time_until_boundary() * UPDATE_INTERVAL_SAFETY_FRACTION;

        self.rtree.insert(wp);
        let result = self.slot_map.insert(entity, wp_clone);

        self.slot_keys_by_entity.insert(entity, result);

        (result, LocationWriteResponse::new(next_update_time, None))
    }

    /// Update the entity with the given key to have a velocity that takes it
    /// from its current location towards the given target at the given speed
    pub fn update_movement_toward(
        &mut self,
        key: &LocationKey,
        target: &Point,
        speed: f64
    ) -> Option<LocationWriteResponse>
    {
        if let Some(wp) = self.slot_map.get_mut(key) {
            let now = self.time.current_time();

            let curr = wp.get_center_at_time(now);
            let dist = curr.distance_to(target);

            Some(self.recenter_window_if_needed(wp, now))
        }
        else {
            None
        }
    }

    /// Update the location of the entity with the given key, and optionally
    /// update the velocity
    pub fn update_by_key(
        &mut self,
        key: &LocationKey,
        new_velocity_opt: Option<Point>
    ) -> Option<LocationWriteResponse> {
        if let Some(wp) = self.slot_map.get_mut(key) {
            let window = &mut **wp;

            let now = self.time.current_time();

            window.update_center(now);

            if let Some(new_velocity) = new_velocity_opt {
                window.velocity = new_velocity;
            }

            Some(self.recenter_window_if_needed(wp))
        } else {
            None
        }
    }

    /// Remove the entity with the given key
    pub fn remove_by_key(&mut self, key: &LocationKey) -> Option<Point> {
        if let Some(wp_ref) = self.slot_map.remove(key) {
            let _ = self
                .rtree
                .remove(wp_ref)
                .expect("Entity missing from rtree when present in slot map");

            let _ = self.slot_keys_by_entity.remove(&wp_ref.entity);

            Some(wp_ref.get_center_at_time(self.time.current_time()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use tokio::runtime::{Builder, Runtime};

    use super::*;

    impl TimeSource for f64 {
        fn current_time(&self) -> f64 {
            *self
        }
    }

    type TestService = LocationServiceInner<f64>;

    fn create_service() -> TestService {
        TestService::new(0.)
    }

    fn player() -> Entity {
        Entity::Player(Default::default())
    }

    #[test]
    fn test_crud() {
        let mut s = create_service();

        let (key, write_resp) = s.insert(
            Point::new(1., 1.5),
            1.5,
            8.,
            Point::new(2., 1.),
            player(),
        );

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(Point::new(1., 1.5)));

        assert_eq!(vec![player()], s.get_entities_at(&Point::new(1., 1.)));

        s.time = 2.0;

        s.update_by_key(&key, Some(Point::new(0., 0.)));

        assert_eq!(
            Vec::<Entity>::new(),
            s.get_entities_at(&Point::new(1., 1.))
        );

        assert_eq!(vec![player()], s.get_entities_at(&Point::new(5., 3.)));

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(Point::new(5., 3.5)));

        assert_eq!(Some(Point::new(5., 3.5)), s.remove_by_key(&key));

        assert_eq!(
            Vec::<Entity>::new(),
            s.get_entities_at(&Point::new(5., 3.))
        );

        assert_eq!(None, s.get_by_key(&key));
        assert_eq!(None, s.get_by_entity(&player()));

        assert_eq!(0, s.slot_keys_by_entity.len());
        assert_eq!(0, s.slot_map.len());
        assert_eq!(0, s.rtree.size());
    }

    #[test]
    fn test_movement_within_and_out_of_window() {
        let mut s = create_service();

        let (key, write_resp) =
            s.insert(Point::new(1., 1.), 1.5, 8., Point::new(2., 2.), player());

        let r = s.get_by_key(&key);

        let initial_window = s.slot_map.get(&key).unwrap().window;

        s.time = 1.0;

        s.update_by_key(&key, None);

        assert_eq!(vec![player()], s.get_entities_at(&Point::new(3., 3.)));

        assert_eq!(initial_window, s.slot_map.get(&key).unwrap().window);

        s.time = 1000.0;

        s.update_by_key(&key, None);

        assert_ne!(initial_window, s.slot_map.get(&key).unwrap().window);

        s.time = 1001.0;

        assert_eq!(
            vec![player()],
            s.get_entities_at(&Point::new(2003., 2003.))
        );
    }
}
