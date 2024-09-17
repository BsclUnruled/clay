use std::any::{type_name, Any};
use std::fmt::Debug;
use std::ops::Deref;
use super::vm::env::Env;
use super::vm::error;
use super::vm::heap::{Heap, Mark};
use super::vm::promise::{Promise,resolve};
use super::vm::runtime::Vm;
use super::vm::signal::ErrSignal;
pub use std::ops::ControlFlow::{Continue as Go,Break as Stop};

pub trait Meta:Any + 'static {
    fn callable(&self)->bool{false}

    fn gc_for_each(&self,_:fn(&Var)){}

    fn call(&self,env:&Env,_args:&[Var])->Promise{
        resolve(Err(
            error::not_a_func(env.vm())
        ))
    }

    fn as_any(&self) -> &dyn Any;

    fn type_name(&self)->&'static str{
        type_name::<Self>()
    }

    fn get(&self,env:&Env,_:&str)->Promise{
        resolve(env.vm().undef())
    }

    fn set(&self,env:&Env,name:&str,_value:&Var)->Promise{
        resolve(Err(error::set_unsetable(env.vm(), self.type_name(), name)))
    }

    fn def(&self,env:&Env,name:&str,_value:&Var)->Promise{
        resolve(Err(error::def_undefable(env.vm(), self.type_name(),name)))
    }

    fn has(&self,_env:&Env,_name:&str)->bool{
        false
    }

    fn to_str(&self)->String{
        format!("ClayObject@{:p}",self)
    }
}

pub trait ToVar{
    fn to_var(self:Self,vm:Vm) -> Var;
}

impl<T:Meta> ToVar for T{
    fn to_var(self:Self,vm:Vm) -> Var where Self: Sized + 'static{
        Var::new(self,vm)
    }
}

pub type Number = f64;

impl Meta for Number{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Meta for String{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Meta for bool{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct VarPtr {
    pub(crate) mark: Mark,
    pub(crate) value: *mut dyn Meta,
}

impl AsRef<dyn Meta> for VarPtr {
    fn as_ref(&self) -> &dyn Meta {
        unsafe{&*self.value}
    }
}

impl VarPtr {
    pub fn new(heap:&mut Heap, value:impl Meta + 'static) ->*mut Self {
        heap.alloc(Self { mark: Mark::New, value: Box::into_raw(Box::new(value)) })
    }
    
    pub fn get_mark(&self) -> Mark {
        self.mark
    }
    pub fn set_mark(&mut self, mark: Mark) {
        self.mark = mark;
    }

    pub fn cast<T: Meta>(&self) -> ErrSignal<&T>{
        let re = self.as_ref();

        match re.as_any().downcast_ref::<T>(){
            Some(v)=>Ok(v),
            None=>Err(error::cast_error(type_name::<T>(), self.as_ref().type_name()))
        }
    }

    #[cfg(debug_assertions)]
    pub fn is<T:Meta>(&self)->bool{
        use std::panic::panic_any;

        match unsafe {self.value.as_ref()}{
            Some(v)=>v.as_any().is::<T>(),
            None=>panic_any("Error:空指针")
        }
    }
}

impl Deref for VarPtr {
    type Target = dyn Meta;
    fn deref(&self) -> &Self::Target {
        unsafe{&*self.value}
    }
}

impl Drop for VarPtr {
    fn drop(&mut self) {
        drop(unsafe {Box::from_raw(self.value)})
    }
}

unsafe impl Sync for VarPtr {}
unsafe impl Send for VarPtr {}

#[derive( Clone)]
pub struct Var {
    ptr:*mut VarPtr,
}

impl Deref for Var {
    type Target = VarPtr;
    fn deref(&self) -> &Self::Target {
        unsafe{&*self.ptr}
    }
}

impl Var {
    pub fn new(value:impl Meta + 'static,vm:Vm) -> Self {
        Self {
            ptr: VarPtr::new(vm.mut_heap(), value),
        }
    }
}

impl Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Var{{ mark: {:?}, value: {:?} }}",
        self.mark,
        self.deref().to_str())
    }
}

unsafe impl Sync for Var {}
unsafe impl Send for Var {}
