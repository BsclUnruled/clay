use crate::clay::vm::{error::set_unsetable, keys, Code};
use super::{undef::undef, Cross, Var};

#[derive(Debug)]
pub struct Func{
    code:Vec<Code>,
}

impl Var for Func{
    fn get(&self, name:&str)->Cross {
        match name{
            keys::CLASS=>ctor(),
            _=>undef()
        }
    }
    fn set(&self, name:&str, _:Cross) {
        set_unsetable("Func", name)
    }
}

pub struct Args{
    args:Vec<Cross>,
    caller:Cross,
}

impl Args{
    pub fn new(args:Vec<Cross>, caller:Cross)->Self{
        Self{args,caller}
    }
    pub fn ctor(_:&Args)->Cross{
        super::to_cross(Box::new(Self::new(vec![], undef())))
    }
    pub fn get_args(&self)->&Vec<Cross>{
        &self.args
    }
}

impl Var for Args{
    fn get(&self, name:&str)->Cross {
        match name{
            keys::CLASS=>ARGS_CTOR.with(|ctor| ctor.clone()),
            "caller"=>self.caller.clone(),
            _=>undef()
        }
    }
    fn set(&self, name:&str, _:Cross) {
        set_unsetable("Args", name)
    }
}

pub struct Native{
    func:&'static dyn Fn(&Args)->Cross
}

impl Native{
    pub fn new(func:&'static dyn Fn(&Args)->Cross)->Box<Self>{
        Box::new(Self{func})
    }
    pub fn call(&self,args:&Args)->Cross{
        (self.func)(args)
    }
}

impl Var for Native{
    fn get(&self, name:&str)->Cross {
        match name{
            keys::CLASS=>ctor(),
            _=>undef()
        }
    }
    fn set(&self, name:&str, _:Cross) {
        set_unsetable("Func", name)
    }
}

thread_local! {
    static CTOR:Cross = super::to_cross(Native::new(&func_ctor));
    static ARGS_CTOR:Cross = super::to_cross(Native::new(&Args::ctor));
}

pub fn new_ctor(func:&'static dyn Fn(&Args)->Cross)->Cross{
    super::to_cross(Native::new(func))
}

fn func_ctor(_:&Args)->Cross{
    super::to_cross(Box::new(Func{code:vec![]}))
}

pub fn ctor()->Cross{
    CTOR.with(|ctor| ctor.clone())
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