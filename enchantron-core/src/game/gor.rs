use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::Deref;

/// Reference that is owned by the game runtime. This allows common references
/// to be passed around in the game without atomic operations for each copy
pub struct Gor<T> {
    value: *const T,
}

// SAFETY: Game owned pointers can only be accessed as shared ref while
// the game runtime is active/not dropped, and will only be dropped after the
// game runtime is dropped
unsafe impl<T> Send for Gor<T> where T: Send + Sync {}
unsafe impl<T> Sync for Gor<T> where T: Send + Sync {}

impl<T> Gor<T> {
    pub fn new(original: &Box<T>) -> Gor<T> {
        Gor {
            value: original.deref(),
        }
    }
}

impl<T> Clone for Gor<T> {
    fn clone(&self) -> Self {
        Gor { value: self.value }
    }
}

impl<T> Debug for Gor<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.deref().fmt(f)
    }
}

impl<T> Deref for Gor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: This value cannot be written to, and since it is owned by
        // the game runtime context, it can be used by any part of the
        // game's services. Because we make sure the application runtime is
        // shutdown before the references they are associated with are dropped
        unsafe { &*self.value }
    }
}
