use std::cell::{Ref, RefCell, RefMut};
use std::ptr;

/// Structure for sharing references be counting, but instead of uniform
/// reference counting, there is only an owner ref and borrower refs. Borrowers
/// cannot drop the shared object and the owner cannot be dropped (without
/// panicing) while there are active borrowers
pub struct SmeiSharedRef<T> {
    owner: bool,
    p: *const RefCell<T>,
    borrows: usize,
}

impl<T> SmeiSharedRef<T> {
    pub fn new(value: T) -> SmeiSharedRef<T> {
        SmeiSharedRef {
            owner: true,
            p: Box::into_raw(Box::new(RefCell::new(value))),
            borrows: 0,
        }
    }

    pub fn read<'a>(&'a self) -> Option<Ref<'a, T>> {
        unsafe { Some((*self.p).borrow()) }
    }

    pub fn write<'a>(&'a self) -> Option<RefMut<'a, T>> {
        unsafe { Some((*self.p).borrow_mut()) }
    }

    pub fn checkout_ref(&mut self) -> SmeiSharedRef<T> {
        self.borrows += 1;
        SmeiSharedRef {
            owner: false,
            p: self.p,
            borrows: 0,
        }
    }

    pub fn return_ref(&mut self, returned: SmeiSharedRef<T>) {
        debug_assert_eq!(self.p, returned.p);
        self.borrows -= 1;
    }
}

impl<T> Drop for SmeiSharedRef<T> {
    fn drop(&mut self) {
        if self.owner {
            if self.borrows != 0 {
                panic!("Tried to drop a shared ref that is still shared");
            }
            unsafe {
                ptr::drop_in_place(self.p as *mut RefCell<T>);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_semi_shared_ref_sharing() {
        {
            let mut r = SmeiSharedRef::new(1usize);
            let rp1 = r.checkout_ref();
            let rp2 = r.checkout_ref();

            r.return_ref(rp1);
            r.return_ref(rp2);
        }
    }

    #[test]
    #[should_panic]
    fn test_semi_shared_ref_incorrect() {
        {
            let mut r = SmeiSharedRef::new(1usize);
            let rp1 = r.checkout_ref();
            let _ = r.checkout_ref();

            r.return_ref(rp1);
        }
    }

    #[test]
    fn test_semi_shared_ref_read_from_shared() {
        {
            let mut r = SmeiSharedRef::new(1usize);
            let rp1 = r.checkout_ref();
            let rp2 = r.checkout_ref();

            assert_eq!(*rp1.read().unwrap(), 1usize);

            *r.write().unwrap() = 10usize;

            assert_eq!(*rp2.read().unwrap(), 10usize);

            r.return_ref(rp1);
            r.return_ref(rp2);
        }
    }

    #[test]
    fn test_semi_shared_ref_write_from_shared() {
        {
            let mut r = SmeiSharedRef::new(1usize);
            let rp1 = r.checkout_ref();
            let rp2 = r.checkout_ref();

            assert_eq!(*rp1.read().unwrap(), 1usize);

            *rp2.write().unwrap() = 10usize;

            assert_eq!(*r.read().unwrap(), 10usize);

            r.return_ref(rp1);
            r.return_ref(rp2);
        }
    }
}
