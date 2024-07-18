use super::{Cross, Var};

pub mod args;
pub mod coro;
pub mod native;
pub mod script;

pub use args::Args;
//use coro::CodeRunner;
pub use script::Script;
pub use native::Native;

pub type Function = 
    &'static dyn Fn(args::Args)->Cross;

thread_local! {
    static CTOR:Cross = super::to_cross(Native::new(&func_ctor));
}

pub fn new_ctor(func:Function)->Cross{
    super::to_cross(Native::new(func))
}

fn func_ctor(_:args::Args)->Cross{
    super::to_cross(Box::new(Script::new(vec![],None,vec![])))
}

pub fn ctor()->Cross{
    CTOR.with(|ctor| ctor.clone())
}

pub enum Func{
    Native(Native),
    Script(Script),
}

impl Var for Func{
    fn get(&self, name: &str) -> Cross {
        match self {
            Func::Native(n) => n.get(name),
            Func::Script(s) => s.get(name),
        }
    }

    fn set(&self, name: &str, value: Cross) {
        match self {
            Self::Native(n) => n.set(name, value),
            Self::Script(s) => s.set(name, value),
        }
    }
}

impl Func{
    pub fn call(&self, args: Args) -> Cross {
        match self {
            Func::Native(n) => n.call(args),
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