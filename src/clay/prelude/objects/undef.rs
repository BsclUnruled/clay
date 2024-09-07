use std::fmt::Display;
use crate::clay::{var::{ToVar, Virtual}, vm::{env::Env, promise::Promise}};
use crate::clay::{var::Var, vm::{error, runtime::Vm}};

#[derive(Debug)]
pub struct Undef();

impl Display for Undef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "undef")
    }
}

impl Virtual for Undef {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call(&self,env:&Env,_:&[Var])->Promise
    where Self:Sized + 'static{
        Err(
            error::not_a_func(env.vm())
        ).into()
    }
}

pub fn new(vm:Vm)->Var{
    Undef().to_var(vm)
}