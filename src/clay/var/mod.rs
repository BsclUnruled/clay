use std::any::{Any, TypeId};
use std::cell::Cell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use int::Int;
use undef::undef;

use super::vm::gc::Mark;
use super::vm::{self, gc};

pub mod func;
pub mod list;
pub mod undef;
pub mod object;
pub mod int;

pub trait Var: Any {
    fn get(&self, name: &str) -> Cross;
    fn set(&self, name: &str, value: Cross);
}

pub fn to_cross(value: Box<dyn Var>) -> Cross {
    Cross::new(value)
}

pub struct VarBox {
    mark: Cell<Mark>,
    id: usize,
    value: Box<dyn Var>,
}

impl VarBox {
    pub fn new(value: Box<dyn Var>) -> Self {
        Self {
            mark: Cell::new(Mark::New),
            id: vm::gc::get_id(),
            value,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_super(&self) -> Cross {
        self.value.get("--super--")
    }
    pub fn get_class(&self) -> Cross {
        self.value.get("--class--")
    }
    pub fn get_mark(&self) -> Mark {
        self.mark.get()
    }
    pub fn set_mark(&self, mark: Mark) {
        self.mark.set(mark)
    }
    pub fn cast<T: Var>(&self) -> Option<&T> {
        if self.value.type_id() == TypeId::of::<T>() {
            let ptr: *const dyn Var = self.value.as_ref();
            Some(unsafe { &*(ptr as *const T) }) //cum rust
        } else {
            None
        }
    }
}

impl Drop for VarBox {
    fn drop(&mut self) {
        gc::back_id(self.id)
    }
}

impl Deref for VarBox {
    type Target = dyn Var;
    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

#[derive(Debug, Clone)]
pub struct Cross {
    weak: Weak<VarBox>,
}

impl Cross {
    pub fn uncross(&self) -> Rc<VarBox> {
        match self.weak.upgrade() {
            Some(var) => var,
            None=>undef().uncross()
        }
    }

    pub fn new(value: Box<dyn Var>) -> Self {
        Self {
            weak:gc::push_heap(VarBox::new(value)),
        }
    }
}
