use std::any::Any;
use std::cell::Cell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use num_bigint::BigInt;
use super::vm::gc::Mark;
use super::vm::runtime::Vm;
use super::vm::signal::{Abort, ErrSignal, Signal};

pub mod func;
pub mod array;
pub mod undef;
pub mod object;
pub mod string;
pub mod lambda;
pub mod future;

pub trait ToVar:Any + 'static{
    fn to_cross(self:Self,vm:Vm) -> Var where Self:Sized + 'static{
        Var::new(Box::new(self),vm)
    }

    fn gc_iter(&self,_this:&Var) -> ErrSignal<Box<dyn Iterator<Item=Signal> + '_>>
    where Self:Sized + 'static{
        Ok(Box::new(std::iter::empty()))
    }
}

// impl dyn ToVar{
//     #[inline]
//     pub fn is<T: Any>(&self) -> bool {
//         // 获取实例化此函数的类型的 `TypeId`。
//         let t = TypeId::of::<T>();

//         // 在 trait 对象 (`self`) 中获取该类型的 `TypeId`。
//         let concrete = self.type_id();

//         // 比较两个 `TypeId` 的相等性。
//         t == concrete
//     }
// }

// pub struct CrossWrap<T: Any>(T);

// impl<T:'static> ToVar for CrossWrap<T> {
//     // fn to_cross(self) -> Cross {
//     //     Cross::new(Box::new(self))
//     // }
// }

impl ToVar for Var {
    fn to_cross(self,_:Vm) -> Var {
        self
    }
}

impl ToVar for Rc<VarBox> {
    fn to_cross(self,_:Vm) -> Var {
        Var { weak: Rc::downgrade(&self) }
    }
}

impl ToVar for Weak<VarBox> {
    fn to_cross(self,_:Vm) -> Var {
        Var { weak: self }
    }
}

impl ToVar for Box<dyn ToVar> {
    fn to_cross(self,vm:Vm) -> Var {
        Var::new(self,vm)
    }
}

impl ToVar for BigInt{}
impl ToVar for f64{}
impl ToVar for String{}
impl ToVar for bool{}

// impl<T:'static> ToCross for T{
//     fn to_cross(self) -> Cross{
//         Cross::new(Box::new(self))
//     }
// }

pub struct VarBox {
    mark: Cell<Mark>,
    id: usize,
    value: Box<dyn ToVar>,
}

impl VarBox {
    pub fn new(value: Box<dyn ToVar>,vm:Vm) -> Self {
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
    
    // pub fn cast<T:ToVar + 'static>(&self) -> Option<&T> {
    //     // if self.value.type_id() == TypeId::of::<T>() {
    //     //     let ptr: *const dyn Any = self.value.as_ref();
    //     //     Some(unsafe { &*(ptr as *const T) }) //cum rust
    //     // } else {
    //     //     None
    //     // }
    //     self.value.as_ref().downcast_ref::<T>()
    // }

    pub fn cast<T: ToVar>(&self) -> Option<&T> {
        //pub trait ToVar:Any + 'static
        let _hc: &dyn ToVar = self.value.as_ref();
        //<dyn Any>::downcast_ref::<T>(hc as &dyn Any)
        None
    }

    pub fn ptr(&self)->*const Self{
        &*self as *const Self
    }
}

// impl Drop for VarBox {
//     fn drop(&mut self) {
//         gc::back_id(self.id)
//     }
// }

impl Deref for VarBox {
    type Target = dyn ToVar;
    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

unsafe impl Sync for VarBox {}
unsafe impl Send for VarBox {}

#[derive(Debug, Clone)]
pub struct Var {
    weak: Weak<VarBox>,
}

impl Var {
    pub fn unbox(&self) -> Result<Rc<VarBox>,Abort> {
        match self.weak.upgrade() {
            Some(var) => Ok(var),
            None=>//vm.borrow().undef().uncross(vm)
                Err(
                    Abort::ThrowString(
                        format!("Error:变量已被回收({:?})",self as *const Var as *const ())
                    )
                )
        }
    }

    pub fn new(value: Box<dyn ToVar>,vm:Vm) -> Self {
        Self {
            weak:vm.borrow_mut().push_heap(VarBox::new(value,vm)),
        }
    }
}

unsafe impl Sync for Var {}
unsafe impl Send for Var {}
