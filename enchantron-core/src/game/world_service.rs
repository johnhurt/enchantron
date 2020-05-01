use super::{GameEntity, GameEntitySlotKey};
use crate::model::{IPoint, IRect, ISize};
use crate::util::SlotMap;
use rstar::{PointDistance, RTree, RTreeObject};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;

fn get_windowed_offset_for(entity: &GameEntity) -> usize {
    use GameEntity::*;

    match entity {
        Player => 8,
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WindowedPointer(Rc<RefCell<WindowedPointerInner>>);

#[derive(Debug, Clone)]
struct WindowedPointerInner {
    entity: GameEntity,
    location: IRect,
    window: IRect,
}

impl PartialEq for WindowedPointerInner {
    fn eq(&self, other: &WindowedPointerInner) -> bool {
        self.entity == other.entity
    }
}

impl WindowedPointer {
    fn new(entity: GameEntity, location: IPoint) -> WindowedPointer {
        let location = IRect {
            top_left: location,
            size: ISize::new(1, 1),
        };
        let window = location.expanded_by(get_windowed_offset_for(&entity));
        WindowedPointer(Rc::new(RefCell::new(WindowedPointerInner {
            entity,
            location,
            window,
        })))
    }

    fn read(&self) -> Ref<WindowedPointerInner> {
        self.0.borrow()
    }

    fn check_window_contains_location(&self) -> bool {
        let inner = self.read();
        inner.window.contains_rect(&inner.location)
    }

    fn write(&self) -> RefMut<WindowedPointerInner> {
        self.0.borrow_mut()
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

impl PointDistance for WindowedPointer {
    /// Returns the squared euclidean distance of an object to a point.
    fn distance_2(&self, point: &IPoint) -> i64 {
        self.0.borrow().location.distance_squared(point)
    }

    /// Returns true if a point is contained within this object.
    fn contains_point(&self, point: &IPoint) -> bool {
        self.0.borrow().location.contains_point(point)
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

impl RTreeObject for WindowedPointer {
    type Envelope = IRect;

    fn envelope(&self) -> Self::Envelope {
        self.read().window
    }
}

pub struct WorldService {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    rtree: RTree<WindowedPointer>,
    slot_map: SlotMap<GameEntitySlotKey, GameEntity, WindowedPointer>,
    slot_keys_by_entity: HashMap<GameEntity, GameEntitySlotKey>,
}

impl WorldService {
    fn new() -> WorldService {
        WorldService {
            inner: Arc::new(RwLock::new(Inner::new())),
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

    pub async fn insert(
        &self,
        e: GameEntity,
        location: IPoint,
    ) -> GameEntitySlotKey {
        self.with_inner_mut(|inner| inner.insert(e, location)).await
    }

    /// Get the current position for the given key
    pub async fn get_by_key(&self, key: &GameEntitySlotKey) -> Option<IRect> {
        self.with_inner(|inner| inner.get_by_key(key)).await
    }

    pub async fn move_by_key(
        &self,
        key: &GameEntitySlotKey,
        new_location: IPoint,
    ) {
        self.with_inner_mut(|inner| inner.move_by_key(key, new_location))
            .await
    }

    pub async fn get_entities_at(&self, point: &IPoint) -> Vec<GameEntity> {
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

    fn insert(&mut self, e: GameEntity, location: IPoint) -> GameEntitySlotKey {
        let wp = WindowedPointer::new(e, location);
        let wp_clone = wp.clone();

        self.rtree.insert(wp);
        let result = self.slot_map.insert(e, wp_clone);

        self.slot_keys_by_entity.insert(e, result);

        result
    }

    fn get_by_key(&self, key: &GameEntitySlotKey) -> Option<IRect> {
        self.slot_map
            .get(key)
            .map(WindowedPointer::read)
            .map(|wp| wp.location)
    }

    fn get_by_entity(
        &self,
        entity: &GameEntity,
    ) -> Option<(GameEntitySlotKey, IRect)> {
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

    fn move_by_key(&mut self, key: &GameEntitySlotKey, new_location: IPoint) {
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

    fn get_entities_at(&self, point: &IPoint) -> Vec<GameEntity> {
        self.rtree
            .locate_all_at_point(point)
            .map(|wp| wp.0.borrow().entity)
            .collect()
    }

    fn remove_by_key(&mut self, key: &GameEntitySlotKey) -> Option<IRect> {
        if let Some(wp_ref) = self.slot_map.remove(key) {
            let wp = self
                .rtree
                .remove(wp_ref)
                .expect("Entity missing from rtree when present in slot map");

            let _ = self.slot_keys_by_entity.remove(&wp.0.borrow().entity);
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

    #[test]
    fn test_crud() {
        let mut s = create_service();

        let key = s.insert(GameEntity::Player, IPoint::new(1, 1));

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(IRect::new(1, 1, 1, 1)));

        assert_eq!(
            vec![GameEntity::Player],
            s.get_entities_at(&IPoint::new(1, 1))
        );

        s.move_by_key(&key, IPoint::new(2, 2));

        assert_eq!(
            Vec::<GameEntity>::new(),
            s.get_entities_at(&IPoint::new(1, 1))
        );

        assert_eq!(
            vec![GameEntity::Player],
            s.get_entities_at(&IPoint::new(2, 2))
        );

        let r = s.get_by_key(&key);

        assert_eq!(r, Some(IRect::new(2, 2, 1, 1)));

        assert_eq!(Some(IRect::new(2, 2, 1, 1)), s.remove_by_key(&key));

        assert_eq!(
            Vec::<GameEntity>::new(),
            s.get_entities_at(&IPoint::new(2, 2))
        );

        assert_eq!(None, s.get_by_key(&key));
        assert_eq!(None, s.get_by_entity(&GameEntity::Player));

        assert_eq!(0, s.slot_keys_by_entity.len());
        assert_eq!(0, s.slot_map.len());
        assert_eq!(0, s.rtree.size());
    }

    #[test]
    fn test_movement_within_and_out_of_window() {
        let mut s = create_service();

        let key = s.insert(GameEntity::Player, IPoint::new(1, 1));

        let initial_window = s.slot_map.get(&key).unwrap().read().window;

        s.move_by_key(&key, IPoint::new(2, 2));

        assert_eq!(
            vec![GameEntity::Player],
            s.get_entities_at(&IPoint::new(2, 2))
        );

        assert_eq!(initial_window, s.slot_map.get(&key).unwrap().read().window);

        s.move_by_key(&key, IPoint::new(1000, 1000));

        assert_ne!(initial_window, s.slot_map.get(&key).unwrap().read().window);

        assert_eq!(
            vec![GameEntity::Player],
            s.get_entities_at(&IPoint::new(1000, 1000))
        );
    }
}
