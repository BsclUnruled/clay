use super::ToVar;
use crate::clay::{var::Var, vm::runtime::Vm};

#[derive(Debug)]
pub struct Undef();

impl ToVar for Undef {}

pub fn new(vm:Vm)->Var{
    Undef().to_cross(vm)
}

// pub fn test() {
//     let ud = undef();
//     let ud2 = ud.uncross();
//     let ud3 = ud2.cast::<Undef>();
//     println!("{:?}", ud3);
// }