use crate::clay::{var::{undef, Cross, Var}, vm::{error::set_unsetable, keys}};

use super::{args,ctor, Function};

pub struct Native{
    func:Function
}

impl Native{
    pub fn new(func:Function)->Box<Self>{
        Box::new(Self{func})
    }
    pub fn call(&self, args:args::Args)->Cross{(self.func)(args)}
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