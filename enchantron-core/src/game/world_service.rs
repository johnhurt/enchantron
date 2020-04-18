use super::WorldEntity;
use crate::model::IRect;
use rstar::{RTree, RTreeObject};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;

struct WorldPointer(Rc<RefCell<(WorldEntity, IRect)>>);

impl RTreeObject for WorldPointer {
    type Envelope = IRect;

    fn envelope(&self) -> Self::Envelope {
        self.1
    }
}

impl Deref for WorldPointer {
    type Target = (WorldEntity, IRect);

    fn deref(&self) -> &(WorldEntity, IRect) {
        self.0.borrow().deref()
    }
}

pub struct WorldService {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    rtree: RTree<WorldPointer>,
}
