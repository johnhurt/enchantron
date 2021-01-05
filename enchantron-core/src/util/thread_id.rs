// Straight up copy of https://github.com/Amanieu/thread_local-rs/blob/master/src/thread_id.rs

use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::usize;

// Thread ID manager which allocates thread IDs. It attempts to aggressively
// reuse thread IDs where possible to avoid cases where a ThreadLocal grows
// indefinitely when it is used by many short-lived threads.
struct ThreadIdManager {
    created_ids: usize,
    free_list: BinaryHeap<usize>,
}
impl ThreadIdManager {
    fn new() -> ThreadIdManager {
        ThreadIdManager {
            created_ids: 0,
            free_list: BinaryHeap::new(),
        }
    }
    fn alloc(&mut self) -> usize {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            let id = self.created_ids;
            self.created_ids = self
                .created_ids
                .checked_add(1)
                .expect("Ran out of thread IDs");
            id
        }
    }
    fn free(&mut self, id: usize) {
        self.free_list.push(id);
    }
}
lazy_static! {
    static ref THREAD_ID_MANAGER: Mutex<ThreadIdManager> =
        Mutex::new(ThreadIdManager::new());
}

// Non-zero integer which is unique to the current thread while it is running.
// A thread ID may be reused after a thread exits.
pub struct ThreadId(usize);
impl ThreadId {
    fn new() -> ThreadId {
        ThreadId(THREAD_ID_MANAGER.lock().unwrap().alloc())
    }
}
impl Drop for ThreadId {
    fn drop(&mut self) {
        THREAD_ID_MANAGER.lock().unwrap().free(self.0);
    }
}
thread_local!(static THREAD_ID: ThreadId = ThreadId::new());

/// Returns a non-zero ID for the current thread
pub fn thread_id() -> usize {
    THREAD_ID.with(|x| x.0)
}
