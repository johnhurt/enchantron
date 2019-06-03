use std::collections::VecDeque;
use std::iter::FilterMap;
use std::slice::Iter;

/// Simple and naive map that keeps elements in slots that can be accessed by
/// id and that id is guaranteed not to change over time
pub struct SimpleSlotMap<T> {
    storage: Vec<Option<T>>,
    queue: VecDeque<usize>,
}

impl<T> Default for SimpleSlotMap<T> {
    fn default() -> SimpleSlotMap<T> {
        SimpleSlotMap {
            storage: Vec::default(),
            queue: VecDeque::default(),
        }
    }
}

impl<T> SimpleSlotMap<T> {
    /// Create a new default simple slot map
    pub fn new() -> SimpleSlotMap<T> {
        SimpleSlotMap::default()
    }

    pub fn len(&self) -> usize {
        self.storage.len() - self.queue.len()
    }

    /// insert the given item into the slot map and return its key
    pub fn insert(&mut self, item: T) -> usize {
        match self.queue.pop_front() {
            Some(empty_slot_index) => {
                if let Some(empty_slot_opt) =
                    self.storage.get_mut(empty_slot_index)
                {
                    if empty_slot_opt.is_some() {
                        error!("Stored empty slot was not actually empty");
                        panic!("Stored empty slot was not actually empty");
                    }

                    *empty_slot_opt = Some(item);
                } else {
                    error!("Stored empty slot did not exist");
                    panic!("Stored empty slot did not exist");
                }
                empty_slot_index
            }
            None => {
                self.storage.push(Some(item));
                self.storage.len() - 1
            }
        }
    }

    /// Remove the item at the given index
    pub fn remove(&mut self, key: usize) {
        if let Some(slot) = self.storage.get_mut(key) {
            if slot.is_none() {
                error!("Tried to remove from an empty slot - {}", key);
                panic!("Tried to remove from an empty slot");
            } else {
                *slot = None;
            }
        } else {
            error!("Tried to remove a non-existant slot - {}", key);
            panic!("Tried to remove a non-existant slot")
        }
        self.queue.push_back(key);
    }

    pub fn iter(
        &self,
    ) -> FilterMap<
        Iter<'_, Option<T>>,
        for<'r> fn(&'r Option<T>) -> Option<&'r T>,
    > {
        self.storage.iter().filter_map(Option::as_ref)
    }
}
