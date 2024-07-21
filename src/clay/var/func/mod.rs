use crate::clay::vm::{signal::Signal, Code};

use super::{Cross, ToCross};

//pub mod args;
//pub mod coro;
//pub mod native;
pub mod script;

//pub use args::Args;
//use coro::CodeRunner;
pub use script::Script;
//pub use native::Native;

pub type Args<'l> = &'l [Code];

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
    Native(Function),
    Script(Script),
    // Functor
    // Coroutine
}

impl ToCross for Func{
    fn to_cross(self)->Cross{
        Cross::new(
            Box::new(self)
        )
    }
}

impl Func{
    pub fn call(&self, args: Args) -> Signal{
        match self {
            Func::Native(n) => n(args),
            Func::Script(s) => s.call(args),
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