use super::{GameEntity, GameEntitySlotKey};
use crate::model::{IPoint, IRect, ISize};
use crate::util::SlotMap;
use rstar::{RTree, RTreeObject};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct WorldPointer(Rc<RefCell<(GameEntity, IRect)>>);

impl WorldPointer {
    fn new(entity: GameEntity, location: IPoint) -> WorldPointer {
        WorldPointer(Rc::new(RefCell::new((
            entity,
            IRect {
                top_left: location,
                size: ISize::new(1, 1),
            },
        ))))
    }

    fn read(&self) -> Ref<(GameEntity, IRect)> {
        self.0.borrow()
    }

    //fn read_rect<'a>(&'a self) -> Ref<'a, ()>

    fn write(&self) -> RefMut<(GameEntity, IRect)> {
        self.0.borrow_mut()
    }
}

impl RTreeObject for WorldPointer {
    type Envelope = IRect;

    fn envelope(&self) -> Self::Envelope {
        self.read().1
    }
}

pub struct WorldService {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    rtree: RTree<WorldPointer>,
    slot_map: SlotMap<GameEntitySlotKey, GameEntity, WorldPointer>,
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
        self.with_inner(move |inner| inner.get_by_key(key)).await
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
        let wp = WorldPointer::new(e, location);
        let wp_clone = wp.clone();

        self.rtree.insert(wp);
        let result = self.slot_map.insert(e, wp_clone);

        self.slot_keys_by_entity.insert(e, result);

        result
    }

    fn get_by_key(&self, key: &GameEntitySlotKey) -> Option<IRect> {
        self.slot_map
            .get(key)
            .map(WorldPointer::read)
            .map(|wp| wp.1)
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
}

#[cfg(test)]
mod test {

    use super::*;

    fn create_service() -> Inner {
        Inner::new()
    }

    #[test]
    fn testCrud() {
        let mut s = create_service();

        s.insert(GameEntity::Player, IPoint::new(1, 1));
    }
}
