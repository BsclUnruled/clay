use std::cell::UnsafeCell;
// use std::ops::Deref;
// use std::rc::Rc as StdRc;

// pub struct Rc<T>(StdRc<T>);

// impl<T> Deref for Rc<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         self.0.as_ref()
//     }
// }

pub struct Cell<T> {//我真无语
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

    pub fn borrow_mut<'s>(&'s self) -> &'s mut T {
        unsafe {
            &mut *self.inner.get()
        }
    }
}

impl<T> Cell<Option<T>>{
    pub const fn none()-> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T {
        let hc = self.borrow_mut();
        if let Some(value) = hc.as_ref() {
            value
        } else {
            let value = f();
            *self.borrow_mut() = Some(value);
            self.borrow().as_ref().expect("Error: 无法在初始化后获取值(from clay::Cell::get_or_init)")
        }
    }
}

unsafe impl<T: Send> Send for Cell<T> {}
unsafe impl<T: Sync> Sync for Cell<T> {}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

unsafe impl<L: Send, R: Send> Send for Either<L, R> {}
unsafe impl<L: Sync, R: Sync> Sync for Either<L, R> {}

pub mod var;
pub mod vm;
pub mod parse;
pub mod prelude;