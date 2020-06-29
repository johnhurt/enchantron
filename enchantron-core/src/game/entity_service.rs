use std::sync::Arc;
use ev_slotmap::{new, ReadHandle, WriteHandle};

pub struct EntityService {
    inner: Arc<Inner>
}

struct Inner {

}