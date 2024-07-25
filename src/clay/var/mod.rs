use std::any::Any;
use std::cell::Cell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use num_bigint::BigInt;

use super::vm::gc::Mark;
use super::vm::runtime::Vm;
use super::vm::signal::Abort;

pub mod func;
pub mod array;
pub mod undef;
pub mod object;
pub mod string;
pub mod lambda;
//pub mod future;

// pub trait Var: Any {
//     fn get(&self, name: &str) -> Cross;
//     fn set(&self, name: &str, value: Cross);
// }

// pub fn to_cross(value: Box<dyn Var>) -> Cross {
//     Cross::new(value)
// }

// impl<T> From<T> for Cross{
//     fn from(value: T) -> Self {
//         Cross::new(Box::new(value))
//     }
// }

pub trait ToCross{
    fn to_cross(self:Self,vm:Vm) -> Cross where Self:Sized + 'static{
        Cross::new(Box::new(self),vm)
    }
}

pub struct CrossWrap<T: Any>(T);

impl<T:'static> ToCross for CrossWrap<T> {
    // fn to_cross(self) -> Cross {
    //     Cross::new(Box::new(self))
    // }
}

impl ToCross for Cross {
    fn to_cross(self,_:Vm) -> Cross {
        self
    }
}

impl ToCross for Rc<VarBox> {
    fn to_cross(self,_:Vm) -> Cross {
        Cross { weak: Rc::downgrade(&self) }
    }
}

impl ToCross for Weak<VarBox> {
    fn to_cross(self,_:Vm) -> Cross {
        Cross { weak: self }
    }
}

impl ToCross for Box<dyn Any> {
    fn to_cross(self,vm:Vm) -> Cross {
        Cross::new(self,vm)
    }
}

impl ToCross for BigInt{}
impl ToCross for f64{}
impl ToCross for String{}
impl ToCross for bool{}

// impl<T:'static> ToCross for T{
//     fn to_cross(self) -> Cross{
//         Cross::new(Box::new(self))
//     }
// }

pub struct VarBox {
    mark: Cell<Mark>,
    id: usize,
    value: Box<dyn Any>,
}

impl VarBox {
    pub fn new(value: Box<dyn Any>,vm:Vm) -> Self {
        Self {
            mark: Cell::new(Mark::New),
            id: vm.borrow_mut().get_id(),
            value,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    // pub fn get_super(&self) -> Cross {
    //     self.value.get("--super--")
    // }
    // pub fn get_class(&self) -> Cross {
    //     self.value.get("--class--")
    // }
    pub fn get_mark(&self) -> Mark {
        self.mark.get()
    }
    pub fn set_mark(&self, mark: Mark) {
        self.mark.set(mark)
    }
    pub fn cast<T:ToCross + 'static>(&self) -> Option<&T> {
        // if self.value.type_id() == TypeId::of::<T>() {
        //     let ptr: *const dyn Any = self.value.as_ref();
        //     Some(unsafe { &*(ptr as *const T) }) //cum rust
        // } else {
        //     None
        // }
        self.value.downcast_ref::<T>()
    }
}

// impl Drop for VarBox {
//     fn drop(&mut self) {
//         gc::back_id(self.id)
//     }
// }

impl Deref for VarBox {
    type Target = dyn Any;
    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

#[derive(Debug, Clone)]
pub struct Cross {
    weak: Weak<VarBox>,
}

impl Cross {
    pub fn uncross(&self) -> Result<Rc<VarBox>,Abort> {
        match self.weak.upgrade() {
            Some(var) => Ok(var),
            None=>//vm.borrow().undef().uncross(vm)
                Err(
                    Abort::ThrowString(
                        format!("Error:变量已被回收({:?})",self as *const Cross as *const ())
                    )
                )
        }
    }

    pub fn new(value: Box<dyn Any>,vm:Vm) -> Self {
        Self {
            weak:vm.borrow_mut().push_heap(VarBox::new(value,vm)),
        }
    }
}
