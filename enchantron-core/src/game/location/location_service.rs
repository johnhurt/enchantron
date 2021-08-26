use super::{MovementRequest, MovementResponse, WindowedLocation};
use crate::game::{Entity, Gor, LocationKey, Time, TimeSource};
use crate::model::{IPoint, IRect, ISize, Point, Rect};
use one_way_slot_map::SlotMap;
use rstar::{PointDistance, RTree, RTreeObject};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;
use tokio::sync::RwLock;

// This value determines the window size for an entity based on its max speed
pub const MAX_SECS_BETWEEN_REINSERTS: f64 = 8.;

// This value determines how close to the edge of the window an entity is
// allowed to get before the window is moved
pub const MIN_SECS_BETWEEN_REINSERTS: f64 = 0.5;

// This is the distance limit at which two points are considered coincident
// if they are closer than this distance (for the purposes of calculating
// offsets)
pub const MINIMUM_DISTANCE: f64 = 0.001;

// If an update to movement is given that has a speed less than this value, the
// speed is considered zero, so the targeted entity will be given no motion
pub const MINIMUM_SPEED: f64 = 0.0001;

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

    /// Insert the entity at the given position and with the given
    /// characteristics. The entity will have no movement when inserted
    pub fn insert(
        &mut self,
        center: Point,
        radius: f64,
        max_speed: f64,
        entity: Entity
    ) -> LocationKey {
        let wp : WindowedLocationPointer = WindowedLocation::new(
            entity, center, radius, max_speed,
        ).into();

        let wp_clone = wp.clone();

        self.rtree.insert(wp);
        let result = self.slot_map.insert(entity, wp_clone);

        self.slot_keys_by_entity.insert(entity, result);

        result
    }

    /// Update the entity with the given key to have movement determined by
    /// the given request
    pub fn update_movement(
        &mut self,
        key: &LocationKey,
        req: MovementRequest
    ) -> Option<MovementResponse>
    {
        if let Some(wp) = self.slot_map.get_mut(key) {
            let new_ref_time = self.time.current_time();
            let new_ref_center = wp.get_center_at_time(new_ref_time);

            // Sanitize the movement request so that we don't have to worry
            // about divide-by-zeros in speed or distance
            let req = req.sanitize(&new_ref_center);

            let (reinsert, result) = match req {
                MovementRequest::Stop => {
                    wp.stop_movement(new_ref_time, new_ref_center)
                }
                MovementRequest::Maintain => {
                    todo!()
                }
                MovementRequest::MoveToward { target, speed } => {
                    let offset = &target - &new_ref_center;
                    let distance = offset.len();
                    let new_velocity = offset * (speed / distance);
                    let time_to_target = distance / speed;

                    wp.upsert_movement(
                        new_ref_time,
                        new_ref_center,
                        target,
                        new_velocity,
                        time_to_target)
                }
            };

            if reinsert {
                // magic words to remove the entity's windowed location from the
                // rtree and reinsert it.
                self.rtree.remove(wp).map(|owp| self.rtree.insert(owp));
            }

            Some(result)
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

        let start = Point::new(1., 1.5);

        let key = s.insert(start, 1.5, 8., player());

        let target = Point::new(2., 1.);
        let speed = 0.5;
        let time_to_target = target.distance_to(&start) / speed;

        // Add motion
        let resp = s.update_movement_target(&key, &target, speed);

        let actual_time_to_target =
            if let Some(LocationWriteResponse::MaintenanceNeeded { time }) =
                resp
            {
                assert!((time - time_to_target).abs() < 0.00001);
                time
            } else {
                panic!("Response should have been - MaintenanceNeeded");
            };

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(Point::new(1., 1.5)));

        assert_eq!(vec![player()], s.get_entities_at(&Point::new(1., 1.)));

        s.time = time_to_target + 0.001;

        assert_eq!(Some(target), s.get_by_key(&key));

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
