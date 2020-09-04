use super::thread_id;

/// Really specific implementation of a ThreadLocal struct that
/// 1. Doesn't allow mutation of entries,
/// 2. Doesn't allow mutation of the underlying store of entries
/// 3. Requires all instances for all threads on construction
#[derive(derive_new::new, Debug)]
pub struct ImmutableThreadLocal<T>
where
    T: Send,
{
    per_thread_values: Vec<T>,
}

unsafe impl<T> Sync for ImmutableThreadLocal<T> where T: Send {}

impl<T> ImmutableThreadLocal<T>
where
    T: Send,
{
    pub fn get(&self) -> &T {
        self.per_thread_values
            .get(thread_id())
            .expect("Thread id out of range")
    }
}
