use std::any::{type_name, Any};
use std::cell::Cell as StdCell;
use std::fmt::{self, Debug};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use super::prelude::objects::args::Args;
use super::vm::error;
use super::vm::gc::Mark;
use super::vm::runtime::Vm;
use super::vm::signal::{Abort, ErrSignal, Signal};
pub use std::ops::ControlFlow::{Continue as Go,Break as Stop};

pub trait Virtual:Any + 'static {
    fn ptr(&self)->String{format!("{:p}",self)}

    fn callable(&self)->bool{false}

    fn gc_for_each(&self,_:fn(&Var)){}

    fn call(&self,all:Args)->Signal{
        Err(
            error::not_a_func(*all.vm())
        )
    }

    fn as_any(&self) -> &dyn Any;

    fn type_name(&self)->&str{
        type_name::<Self>()
    }

    fn get(&self,vm:Vm,_:&str)->Signal{
        vm.undef()
    }

    fn set(&self,vm:Vm,name:&str,_value:&Var)->Signal{
        Err(error::set_unsetable(vm, self.type_name(), name))
    }

    fn def(&self,vm:Vm,name:&str,_value:&Var)->Signal{
        Err(error::def_undefable(vm, self.type_name(),name))
    }

    fn has(&self,_vm:Vm,_name:&str)->bool{
        false
    }
}

pub trait ToVar{
    fn to_var(self:Self,vm:Vm) -> Var;
    fn to_varbox(self:Self,vm:Vm) -> VarBox;
}

impl<T:Virtual> ToVar for T{
    fn to_var(self:Self,vm:Vm) -> Var where Self: Sized + 'static{
        Var::new(Box::new(self),vm)
    }

    fn to_varbox(self:Self,vm:Vm) -> VarBox {
        VarBox::new(Box::new(self),vm)
    }
}

/*impl dyn Virtual {
    /// Returns `true` if the inner type is the same as `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ToVar::ToVar;
    ///
    /// fn is_string(s: &dyn ToVar) {
    ///     if s.is::<String>() {
    ///         println!("It's a string!");
    ///     } else {
    ///         println!("Not a string...");
    ///     }
    /// }
    ///
    /// is_string(&0);
    /// is_string(&"cookie monster".to_string());
    /// ```
    #[inline]
    pub fn is<T: Virtual>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    /// Returns some reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ToVar::ToVar;
    ///
    /// fn print_if_string(s: &dyn ToVar) {
    ///     if let Some(string) = s.downcast_ref::<String>() {
    ///         println!("It's a string({}): '{}'", string.len(), string);
    ///     } else {
    ///         println!("Not a string...");
    ///     }
    /// }
    ///
    /// print_if_string(&0);
    /// print_if_string(&"cookie monster".to_string());
    /// ```
    #[inline]
    pub fn downcast_ref<T: Virtual>(&self) -> Option<&T> {
        #[cfg(debug_assertions)]{
            println!("[Cast] {} -> {} ({})"
               ,type_name::<Self>(),type_name::<T>(),
                if self.is::<T>() { "ok" } else { "failed" });
        }
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented ToVar for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_ref_unchecked()) }
        } else {
            None
        }

        //unsafe { Some(self.downcast_ref_unchecked()) }
    }

    /// Returns some mutable reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ToVar::ToVar;
    ///
    /// fn modify_if_u32(s: &mut dyn ToVar) {
    ///     if let Some(num) = s.downcast_mut::<u32>() {
    ///         *num = 42;
    ///     }
    /// }
    ///
    /// let mut x = 10u32;
    /// let mut s = "starlord".to_string();
    ///
    /// modify_if_u32(&mut x);
    /// modify_if_u32(&mut s);
    ///
    /// assert_eq!(x, 42);
    /// assert_eq!(&s, "starlord");
    /// ```
    #[inline]
    pub fn downcast_mut<T: Virtual>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented ToVar for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_mut_unchecked()) }
        } else {
            None
        }
    }

    /// Returns a reference to the inner value as type `dyn T`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(downcast_unchecked)]
    ///
    /// use std::ToVar::ToVar;
    ///
    /// let x: Box<dyn ToVar> = Box::new(1_usize);
    ///
    /// unsafe {
    ///     assert_eq!(*x.downcast_ref_unchecked::<usize>(), 1);
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// The contained value must be of type `T`. Calling this method
    /// with the incorrect type is *undefined behavior*.
    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: Virtual>(&self) -> &T {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &*(self as *const dyn Virtual as *const T) }
    }

    /// Returns a mutable reference to the inner value as type `dyn T`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(downcast_unchecked)]
    ///
    /// use std::ToVar::ToVar;
    ///
    /// let mut x: Box<dyn ToVar> = Box::new(1_usize);
    ///
    /// unsafe {
    ///     *x.downcast_mut_unchecked::<usize>() += 1;
    /// }
    ///
    /// assert_eq!(*x.downcast_ref::<usize>().unwrap(), 2);
    /// ```
    ///
    /// # Safety
    ///
    /// The contained value must be of type `T`. Calling this method
    /// with the incorrect type is *undefined behavior*.
    #[inline]
    pub unsafe fn downcast_mut_unchecked<T: Virtual>(&mut self) -> &mut T {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &mut *(self as *mut dyn Virtual as *mut T) }
    }
}
*/
// impl dyn ToVar{
//     #[inline]
//     pub fn is<T: ToVar>(&self) -> bool {
//         // 获取实例化此函数的类型的 `TypeId`。
//         let t = TypeId::of::<T>();

//         // 在 trait 对象 (`self`) 中获取该类型的 `TypeId`。
//         let concrete = self.type_id();

//         // 比较两个 `TypeId` 的相等性。
//         t == concrete
//     }
// }

// impl Virtual for BigInt{
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

pub type Number = f64;

impl Virtual for Number{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Virtual for String{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Virtual for bool{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Coro{
    
}

// pub struct CoroWrap<R:Any,T: Coroutine<R>>(pub T,PhantomData<R>);

// impl<R:Any,T: Coroutine<R>> CoroWrap<R,T>{
//     pub fn new(iter: T) -> Self {
//         Self(iter,PhantomData)
//     }

//     pub fn none() -> Self{
//         let hc = #[coroutine] || {
//             yield 1;
//             return "foo"
//         };
//         Self::new(hc)
//     }
// }

// impl<T:'static + Iterator<Item = Var>>  Virtual for CoroWrap<T>{
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl<T:'static> ToCross for T{
//     fn to_cross(self) -> Cross{
//         Cross::new(Box::new(self))
//     }
// }

pub struct VarBox {
    pub(crate) mark: StdCell<Mark>,
    pub(crate) id: usize,
    pub(crate) value: Box<dyn Virtual>,
    //pub(crate) constuctor:Var
}

impl VarBox {
    // pub fn global_context() -> Self {
    //     let global = HashMap::new();
        
    //     Self{
    //         mark:StdCell::new(Mark::New),
    //         id:0,
    //         value:Box::new(env::Ctx(crate::clay::Cell::new(global), Some(env::void_ctx())))
    //     }
    // }
    pub fn new(value: Box<dyn Virtual>,vm:Vm) -> Self {
        Self {
            mark: StdCell::new(Mark::New),
            id: vm.get_id(),
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
    //     //     let ptr: *const dyn ToVar = self.value.as_ref();
    //     //     Some(unsafe { &*(ptr as *const T) }) //cum rust
    //     // } else {
    //     //     None
    //     // }
    //     self.value.as_ref().downcast_ref::<T>()
    // }

    pub fn cast<T: Virtual>(&self) -> ErrSignal<&T>{
        match self.value.as_ref().as_any().downcast_ref::<T>(){
            Some(v)=>Ok(v),
            None=>Err(error::cast_error(type_name::<T>(), self.value.as_ref().type_name()))
        }
    }

    #[cfg(debug_assertions)]
    pub fn is<T:Virtual>(&self)->bool{
        self.value.as_any().is::<T>()
    }
}

impl Deref for VarBox {
    type Target = dyn Virtual;
    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

unsafe impl Sync for VarBox {}
unsafe impl Send for VarBox {}

#[derive( Clone)]
pub struct Var {
    pub(crate) weak: Weak<VarBox>,
}

impl Var {
    pub fn unbox(&self) -> Result<Rc<VarBox>,Abort> {
        match self.weak.upgrade() {
            Some(var) => Ok(var),
            None=>//vm.borrow().undef().uncross(vm)
                Err(
                    Abort::ThrowString(
                        format!("Error:变量已被回收({:p})",self)
                    )
                )
        }
    }

    pub fn new(value: Box<dyn Virtual>,vm:Vm) -> Self {
        Self {
            weak:vm.push_heap(VarBox::new(value,vm)),
        }
    }
}

impl Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Var{{ id: {}, mark: {:?}, value: {:?} }}",
        self.unbox().unwrap().id,
        self.unbox().unwrap().mark.get(),
        match self.unbox(){
            Ok(_)=>"todo",
            Err(_)=>Err(fmt::Error{})?
        })
    }
}

unsafe impl Sync for Var {}
unsafe impl Send for Var {}
