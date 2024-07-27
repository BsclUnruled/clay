use std::rc::Rc;
use crate::clay::vm::runtime::Vm;
use crate::clay::vm::env::Context;
use crate::clay::vm::{signal::Signal, Token};
use super::ToVar;

//pub mod args;
//pub mod coro;
//pub mod native;
pub mod script;
pub use script::Script;


pub type Args<'l> = (
    Vm,
    &'l [Token],
    Rc<dyn Context>,
    // &'l Yielder<Var, Signal>,
);

pub type Function = 
    &'static dyn Fn(Args)->Signal;

// thread_local! {
//     static CTOR:Cross = Func::Native(&func_ctor).to_cross();
// }

// pub fn new_ctor(func:Function)->Cross{
//     Func::Native(func).to_cross()
// }

// // pub fn from_lambda(lam:&impl Fn(Args)->Cross)->Cross{
// //     Func::Native(Box::leak(
// //         Box::new(lam)
// //     )).to_cross()
// // }


// pub fn ctor()->Cross{
//     CTOR.with(|ctor| ctor.clone())
// }

pub enum Func{
    Native(Function,String),
    Script(Script),
    //Coro(Coro)
    // Functor
}

impl ToVar for Func{}

impl Func{
    pub fn call(&self, args: Args) -> Signal{
        match self {
            Func::Native(n,_) => n(args),
            Func::Script(s) => s.call(args),
            //Func::Coro(f)=> f.resume(args)
        }
    }

    pub fn name(&self)->&str{
        match self {
            Func::Native(_,name) => name,
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