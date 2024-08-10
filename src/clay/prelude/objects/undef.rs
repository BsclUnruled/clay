use std::fmt::Display;
use crate::clay::var::{ToVar, Virtual};
use super::args::Args;
use crate::clay::{var::Var, vm::{error, runtime::Vm, signal::Signal}};

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
    fn call(&self,all:Args)->Signal
    where Self:Sized + 'static{
        Err(
            error::not_a_func(*all.vm())
        )
    }
}

pub fn new(vm:Vm)->Var{
    Undef().to_var(vm)
}

// pub fn test() {
//     let ud = undef();
//     let ud2 = ud.uncross();
//     let ud3 = ud2.cast::<Undef>();
//     println!("{:?}", ud3);
// }