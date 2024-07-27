use super::{Virtual, Var};
use crate::clay::vm::env::Context;
use crate::clay::vm::runtime::Vm;
use crate::clay::vm::signal::Signal;
use std::fmt::{Debug, Display};
use std::rc::Rc;

//pub mod args;
//pub mod coro;
//pub mod native;
pub mod script;
pub use script::Script;

pub type Args<'l> = (
    Vm,
    &'l [Var],
    Rc<dyn Context>,
    // &'l Yielder<Var, Signal>,
);

pub type Function = &'static dyn Fn(Args) -> Signal;

pub enum Func {
    Native(Function, String),
    Script(Script),
    //Coro(Coro)
    // Functor
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::Native(ptr, name) => 
                write!(f, "NativeFunc {}@{:p}", name, ptr as *const _),
            Func::Script(s) => write!(f, "Func {}", s.name),
            //Func::Coro(_) => write!(f, "Coro"),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::Native(ptr, name) => 
                write!(f, "NativeFunc {}@{:p}", name, ptr),
            Func::Script(s) => write!(f, "Func {}", s.name),
        }
    }
}

impl Virtual for Func {
    fn as_func(&self, args: Args) -> Signal
    where
        Self: Sized + 'static,
    {
        match self {
            Func::Native(n, _) => n(args),
            Func::Script(s) => s.call(args),
            //Func::Coro(f)=> f.resume(args)
        }
    }
}

impl Func {
    pub fn name(&self) -> &str {
        match self {
            Func::Native(_, name) => name,
            Func::Script(s) => &s.name,
        }
    }
}

/*编写一个宏,为它生成ctor(Cross),thread_local,和导出函数
*fn ctor(_:&Args)->Cross{...}
*
* macro_rules! ctor {
   ($name:ident) => {
       thread_local! {
           static $name:Cross = super::to_cross(Box::new($name{code:vec![]}));
       }
       pub fn $name_ctor() -> Cross {
           $name.with(|ctor| ctor.clone())
       }
   };
 }
*
* */
