use std::cell::UnsafeCell;

pub struct Cell<T> {
    inner: UnsafeCell<T>,
}

impl<T> Cell<T>{
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn borrow(&self) -> &T {
        unsafe {
            &*self.inner.get()
        }
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe {
            &mut *self.inner.get()
        }
    }
}

pub mod var;
pub mod vm;
pub mod parse;
pub mod prelude;