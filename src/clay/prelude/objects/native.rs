use crate::clay::vm::signal::Signal;
use std::fmt::{Debug, Display};

use super::args::Args;

pub type Function = fn(Args) -> Signal;

pub struct Native{
    func:Function,
    name:Option<String>
}

impl Debug for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "NativeFunc {}@{:p}", name, self),
            None => write!(f, "NativeFunc@{:p}", self),
        }
    }
}

impl Display for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "NativeFunc {}@{:p}", name, self),
            None => write!(f, "NativeFunc@{:p}", self),
        }
    }
}

impl Native {
    pub fn name(&self) -> String {
        match &self.name {
            Some(name) => name.to_owned(),
            None => format!("NativeFunc@{:p}", self),
        }
    }

    pub fn call(&self, args:Args) -> Signal {
        (self.func)(args)
    }

    pub fn new(func:Function,name:Option<String>) -> Self {
        Self {
            name,func
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
