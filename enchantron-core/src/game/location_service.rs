use super::{Entity, Gor, LocationKey, LocationWriteResponse, Time};
use crate::model::{IPoint, IRect, ISize, Point};
use one_way_slot_map::SlotMap;
use rstar::{PointDistance, RTree, RTreeObject};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;
use tokio::sync::RwLock;

// This value determines the window size for an entity based on its max speed
const MAX_SECS_BETWEEN_REINSERTS : f64 = 8.;

// This value determines how close to the edge of the window an entity is
// allowed to get before the window is moved
const MIN_SECS_BETWEEN_REINSERTS : f64 = 0.5;

// Movement operations on the location service will provide a time at which the
// entity will need to be updated. This time will be when the entity is
// calculated to exit the window multiplied by this safety factor
const UPDATE_INTERVAL_SAFETY_FRACTION : f64 = 0.75;

// Default


#[derive(Debug, Copy, Clone, derive_new::new)]
pub struct WindowedLocation {
    pub center: Point,
    pub radius: f64,
    pub ref_time: f64,
    pub max_speed: f64,
    pub velocity: Point,
    pub entity: Entity,
    pub window: IRect,
}

#[derive(Debug)]
struct WindowedLocationPointer {
    owner: bool,
    p: *const WindowedLocation,
    _marker: PhantomData<WindowedLocation>,
}

fn get_windowed_offset_for(entity: &Entity) -> usize {
    use Entity::*;

    match entity {
        Player(_) => 8,
    }
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

impl WindowedLocationPointer {
    fn new(
        entity: Entity,
        center: Point,
        velocity: Point,
        radius: f64,
        max_speed: f64,
        ref_time: f64,
        window: IRect
    ) -> WindowedLocationPointer {
        WindowedLocation::new(
            center,
            radius,
            ref_time,
            max_speed,
            velocity,
            entity,
            window).into()
    }

    fn new_with_centered_window(
        entity: Entity,
        center: Point,
        velocity: Point,
        radius: f64,
        max_speed: f64,
        time: &Time) -> WindowedLocationPointer
    {
        let mut result = WindowedLocationPointer::new(
            entity, center, velocity, radius, max_speed, time.now(), IRect::default()
        );

        result.recenter_window();

        result
    }

    fn check_window_contains_location(&self) -> bool {
        let inner = &**self;
        inner.window.contains_point(&inner.center.floor())
    }

    /// Move the window for this windowed pointer to be the rectangle around the
    /// actual location
    fn recenter_window(&mut self) {
        let inner = &mut **self;
        let dist = inner.radius + MAX_SECS_BETWEEN_REINSERTS / inner.max_speed;

        inner.window = IRect {
            top_left: inner.center.floor(),
            size: ISize::new(1, 1)
        };

        inner.window.expanded_by(dist.ceil() as usize);
    }

    /// Get the topmost and leftmost point that the center can be inside the
    /// window.
    fn get_top_left_center_limit(&self) -> Point {
        let result = self.window.top_left.as_point();
        result.x += self.radius;
        result.y += self.radius;
        result
    }

    ///  Get the time until any part of the entity exits the window
    fn time_until_boundary(&self) -> f64 {
        0.0
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
    inner: Gor<RwLock<Inner>>,
}

#[derive(Debug)]
struct Inner {
    time: Time,
    rtree: RTree<WindowedLocationPointer>,
    slot_map: SlotMap<LocationKey, Entity, WindowedLocationPointer>,
    slot_keys_by_entity: HashMap<Entity, LocationKey>,
}

#[allow(dead_code)]
impl LocationService {
    pub fn new(time: Time) -> (LocationService, impl FnOnce() + Send) {
        let boxed_inner = Box::new(RwLock::new(Inner::new(time)));
        let inner = Gor::new(&boxed_inner);

        (LocationService { inner }, move || drop(boxed_inner))
    }

    pub fn new_from_data(
        time: Time,
        data: &SlotMap<LocationKey, Entity, WindowedLocation>,
    ) -> (LocationService, impl FnOnce() + Send) {
        let boxed_inner = Box::new(RwLock::new(Inner::new_from_data(time, data)));
        let inner = Gor::new(&boxed_inner);

        (LocationService { inner }, move || drop(boxed_inner))
    }

    async fn with_inner<T>(&self, action: impl FnOnce(&Inner) -> T) -> T {
        let inner = self.inner.read().await;

        action(&*inner)
    }

    async fn with_inner_mut<T>(
        &self,
        action: impl FnOnce(&mut Inner) -> T,
    ) -> T {
        let mut inner = self.inner.write().await;

        action(&mut *inner)
    }

    pub async fn insert(&self,
        center: Point,
        radius: f64,
        ref_time: f64,
        max_speed: f64,
        velocity: Point,
        entity: Entity) -> (LocationKey,LocationWriteResponse) {
        self.with_inner_mut(|inner| inner.insert(
            center,
            radius,
            ref_time,
            max_speed,
            velocity,
            entity,)).await
    }

    /// Get the current position for the given key
    pub async fn get_by_key(&self, key: &LocationKey) -> Option<Point> {
        self.with_inner(|inner| inner.get_by_key(key)).await
    }

    pub async fn update_by_key(&self, key: &LocationKey, new_center: Point, new_velocity: Point) {
        self.with_inner_mut(|inner| inner.update_by_key(key, new_center, new_velocity))
            .await
    }

    pub async fn move_by_key_delta(&self, key: &LocationKey, shift: &IPoint) {
        self.with_inner_mut(|inner| inner.move_by_key_delta(key, shift))
            .await
    }

    pub async fn get_entities_at(&self, point: &IPoint) -> Vec<Entity> {
        self.with_inner(|inner| inner.get_entities_at(point)).await
    }
}

#[allow(dead_code)]
impl Inner {
    fn new(time: Time) -> Inner {
        Inner {
            time,
            rtree: RTree::new(),
            slot_map: SlotMap::new(),
            slot_keys_by_entity: Default::default(),
        }
    }

    fn new_from_data(
        time: Time,
        data: &SlotMap<LocationKey, Entity, WindowedLocation>,
    ) -> Inner {
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

        Inner {
            time,
            rtree,
            slot_map,
            slot_keys_by_entity,
        }
    }

    fn insert(&mut self,
        center: Point,
        radius: f64,
        ref_time: f64,
        max_speed: f64,
        velocity: Point,
        entity: Entity ) -> (LocationKey, LocationWriteResponse) {
        let wp = WindowedLocationPointer::new_with_centered_window(
            entity,
            center,
            velocity,
            radius,
            max_speed,
            &self.time);
        let wp_clone = wp.clone();

        self.rtree.insert(wp);
        let result = self.slot_map.insert(entity, wp_clone);

        self.slot_keys_by_entity.insert(entity, result);


        (result, LocationWriteResponse::new())
    }

    fn get_by_key(&self, key: &LocationKey) -> Option<Point> {
        self.slot_map
            .get(key)
            .map(WindowedLocationPointer::read)
            .map(|wp| wp.center)
    }

    fn get_by_entity(&self, entity: &Entity) -> Option<(LocationKey, Point)> {
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

    fn update_by_key(&mut self, key: &LocationKey, new_center: Point, new_velocity: Point) {
        if let Some(wp) = self.slot_map.get_mut(key) {
            wp.write().center.top_left = new_location;
            if !wp.check_window_contains_location() {
                let mut owned_wp = self
                    .rtree
                    .remove(wp)
                    .expect("Entity pointer should still be in rtree");
                owned_wp.recenter_window();
                self.rtree.insert(owned_wp);
            }
        }
    }

    fn move_by_key_delta(&mut self, key: &LocationKey, shift: &IPoint) {
        if let Some(wp) = self.slot_map.get_mut(key) {
            wp.write().location.top_left += shift;
            if !wp.check_window_contains_location() {
                let mut owned_wp = self
                    .rtree
                    .remove(wp)
                    .expect("Entity pointer should still be in rtree");
                owned_wp.recenter_window();
                self.rtree.insert(owned_wp);
            }
        }
    }

    fn get_entities_at(&self, point: &IPoint) -> Vec<Entity> {
        self.rtree
            .locate_all_at_point(point)
            .map(|wp| wp.read().entity)
            .collect()
    }

    fn remove_by_key(&mut self, key: &LocationKey) -> Option<Point> {
        if let Some(wp_ref) = self.slot_map.remove(key) {
            let _ = self
                .rtree
                .remove(wp_ref)
                .expect("Entity missing from rtree when present in slot map");

            let _ = self.slot_keys_by_entity.remove(&wp_ref.read().entity);

            Some(wp_ref.read().center)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn create_service() -> Inner {
        Inner::new()
    }

    fn player() -> Entity {
        Entity::Player(Default::default())
    }

    #[test]
    fn test_crud() {
        let mut s = create_service();

        let key = s.insert(player(), IPoint::new(1, 1));

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(IRect::new(1, 1, 1, 1)));

        assert_eq!(vec![player()], s.get_entities_at(&IPoint::new(1, 1)));

        s.move_by_key(&key, IPoint::new(2, 2));

        assert_eq!(Vec::<Entity>::new(), s.get_entities_at(&IPoint::new(1, 1)));

        assert_eq!(vec![player()], s.get_entities_at(&IPoint::new(2, 2)));

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(IRect::new(2, 2, 1, 1)));

        assert_eq!(Some(IRect::new(2, 2, 1, 1)), s.remove_by_key(&key));

        assert_eq!(Vec::<Entity>::new(), s.get_entities_at(&IPoint::new(2, 2)));

        assert_eq!(None, s.get_by_key(&key));
        assert_eq!(None, s.get_by_entity(&player()));

        assert_eq!(0, s.slot_keys_by_entity.len());
        assert_eq!(0, s.slot_map.len());
        assert_eq!(0, s.rtree.size());
    }

    #[test]
    fn test_movement_within_and_out_of_window() {
        let mut s = create_service();

        let key = s.insert(player(), IPoint::new(1, 1));

        let initial_window = s.slot_map.get(&key).unwrap().read().window;

        s.move_by_key(&key, IPoint::new(2, 2));

        assert_eq!(vec![player()], s.get_entities_at(&IPoint::new(2, 2)));

        assert_eq!(initial_window, s.slot_map.get(&key).unwrap().read().window);

        s.move_by_key(&key, IPoint::new(1000, 1000));

        assert_ne!(initial_window, s.slot_map.get(&key).unwrap().read().window);

        assert_eq!(vec![player()], s.get_entities_at(&IPoint::new(1000, 1000)));
    }
}
