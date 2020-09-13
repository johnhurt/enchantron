use super::{Entity, LocationKey};
use crate::model::{IPoint, IRect, ISize};
use one_way_slot_map::SlotMap;
use rstar::{PointDistance, RTree, RTreeObject};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ptr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Copy, Clone, derive_new::new)]
pub struct SaveableLocation {
    pub location: IRect,
    pub entity: Entity,
}

#[derive(Debug)]
struct WindowedPointer {
    owner: bool,
    p: *const WindowedPointerInner,
    _marker: PhantomData<WindowedPointerInner>,
}

fn get_windowed_offset_for(entity: &Entity) -> usize {
    use Entity::*;

    match entity {
        Player(_) => 8,
    }
}

impl PartialEq for WindowedPointer {
    fn eq(&self, other: &WindowedPointer) -> bool {
        self.read().entity == other.read().entity
    }
}

impl RTreeObject for WindowedPointer {
    type Envelope = IRect;

    fn envelope(&self) -> Self::Envelope {
        self.read().window
    }
}

impl PointDistance for WindowedPointer {
    /// Returns the squared euclidean distance of an object to a point.
    fn distance_2(&self, point: &IPoint) -> i64 {
        self.read().location.distance_squared(point)
    }

    /// Returns true if a point is contained within this object.
    fn contains_point(&self, point: &IPoint) -> bool {
        self.read().location.contains_point(point)
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

unsafe impl Send for WindowedPointer {}
unsafe impl Sync for WindowedPointer {}

impl Drop for WindowedPointer {
    fn drop(&mut self) {
        if self.owner {
            unsafe {
                ptr::drop_in_place(self.p as *mut WindowedPointerInner);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct WindowedPointerInner {
    entity: Entity,
    location: IRect,
    window: IRect,
}

impl PartialEq for WindowedPointerInner {
    fn eq(&self, other: &WindowedPointerInner) -> bool {
        self.entity == other.entity
    }
}

impl From<&SaveableLocation> for WindowedPointer {
    fn from(src: &SaveableLocation) -> Self {
        Self::new(src.entity, src.location.top_left)
    }
}

impl WindowedPointer {
    fn new(entity: Entity, location: IPoint) -> WindowedPointer {
        let location = IRect {
            top_left: location,
            size: ISize::new(1, 1),
        };
        let window = location.expanded_by(get_windowed_offset_for(&entity));
        let inner = WindowedPointerInner {
            entity,
            location,
            window,
        };

        WindowedPointer {
            owner: true,
            p: Box::into_raw(Box::new(inner)),
            _marker: Default::default(),
        }
    }

    pub fn read<'a>(&'a self) -> &'a WindowedPointerInner {
        unsafe { &*self.p }
    }

    pub fn write<'a>(&'a mut self) -> &'a mut WindowedPointerInner {
        unsafe { &mut *(self.p as *mut WindowedPointerInner) }
    }

    fn check_window_contains_location(&self) -> bool {
        let inner = self.read();
        inner.window.contains_rect(&inner.location)
    }

    /// Move the window for this windowed pointer to be the rectangle around the
    /// actual location
    fn recenter_window(&mut self) {
        let mut inner = self.write();
        inner.window = inner
            .location
            .expanded_by(get_windowed_offset_for(&inner.entity));
    }
}

impl Clone for WindowedPointer {
    fn clone(&self) -> WindowedPointer {
        WindowedPointer {
            owner: false,
            p: self.p,
            _marker: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocationService {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Debug)]
struct Inner {
    rtree: RTree<WindowedPointer>,
    slot_map: SlotMap<LocationKey, Entity, WindowedPointer>,
    slot_keys_by_entity: HashMap<Entity, LocationKey>,
}

impl LocationService {
    pub fn new() -> LocationService {
        LocationService {
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    pub fn new_from_data(
        data: &SlotMap<LocationKey, Entity, SaveableLocation>,
    ) -> LocationService {
        LocationService {
            inner: Arc::new(RwLock::new(Inner::new_from_data(data))),
        }
    }

    async fn with_inner<T>(&self, action: impl FnOnce(&Inner) -> T) -> T {
        let ref mut inner = self.inner.read().await;

        action(inner)
    }

    async fn with_inner_mut<T>(
        &self,
        action: impl FnOnce(&mut Inner) -> T,
    ) -> T {
        let ref mut inner = self.inner.write().await;

        action(inner)
    }

    pub async fn insert(&self, e: Entity, location: IPoint) -> LocationKey {
        self.with_inner_mut(|inner| inner.insert(e, location)).await
    }

    /// Get the current position for the given key
    pub async fn get_by_key(&self, key: &LocationKey) -> Option<IRect> {
        self.with_inner(|inner| inner.get_by_key(key)).await
    }

    pub async fn move_by_key(&self, key: &LocationKey, new_location: IPoint) {
        self.with_inner_mut(|inner| inner.move_by_key(key, new_location))
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

impl Inner {
    fn new() -> Inner {
        Inner {
            rtree: RTree::new(),
            slot_map: SlotMap::new(),
            slot_keys_by_entity: Default::default(),
        }
    }

    fn new_from_data(
        data: &SlotMap<LocationKey, Entity, SaveableLocation>,
    ) -> Inner {
        let mut rtree = RTree::new();
        let mut slot_keys_by_entity = HashMap::default();

        data.iter(|loc| loc.entity).for_each(|(location_key, loc)| {
            slot_keys_by_entity.insert(loc.entity, location_key);
        });

        let slot_map = data.map(|loc| {
            let wp = WindowedPointer::from(loc);
            let wp_clone = wp.clone();

            rtree.insert(wp);
            wp_clone
        });

        Inner {
            rtree,
            slot_map,
            slot_keys_by_entity,
        }
    }

    fn insert(&mut self, e: Entity, location: IPoint) -> LocationKey {
        let mut wp = WindowedPointer::new(e, location);
        let wp_clone = wp.clone();

        self.rtree.insert(wp);
        let result = self.slot_map.insert(e, wp_clone);

        self.slot_keys_by_entity.insert(e, result);

        result
    }

    fn get_by_key(&self, key: &LocationKey) -> Option<IRect> {
        self.slot_map
            .get(key)
            .map(WindowedPointer::read)
            .map(|wp| wp.location)
    }

    fn get_by_entity(&self, entity: &Entity) -> Option<(LocationKey, IRect)> {
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

    fn move_by_key(&mut self, key: &LocationKey, new_location: IPoint) {
        if let Some(mut wp) = self.slot_map.get_mut(key) {
            wp.write().location.top_left = new_location;
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
        if let Some(mut wp) = self.slot_map.get_mut(key) {
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

    fn remove_by_key(&mut self, key: &LocationKey) -> Option<IRect> {
        if let Some(mut wp_ref) = self.slot_map.remove(key) {
            let _ = self
                .rtree
                .remove(wp_ref)
                .expect("Entity missing from rtree when present in slot map");

            let _ = self.slot_keys_by_entity.remove(&wp_ref.read().entity);

            Some(wp_ref.read().location)
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
