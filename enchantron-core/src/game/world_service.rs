use super::WorldEntity;
use crate::model::IRect;
use rstar::{RTree, RTreeObject};
use slotmap::SlotMap;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct WorldPointer(Rc<RefCell<(WorldEntity, IRect)>>);

impl WorldPointer {
    fn read<'a>(&'a self) -> Ref<'a, (WorldEntity, IRect)> {
        self.0.borrow()
    }

    fn write<'a>(&'a self) -> RefMut<'a, (WorldEntity, IRect)> {
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
    slot_map: SlotMap<WorldEntity, WorldPointer>,
}

impl WorldService {

    async fn withInnerMut<T>(&self, action: impl FnOnce(&mut Inner) -> T) -> T {
        let inner_mut = self.inner.write().await;
        action(&mut *inner_mut)
    }

    pub fn insert(e: WorldEntity, location: IPoint) -> {

    }

}