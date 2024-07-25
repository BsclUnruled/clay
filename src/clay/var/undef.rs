use super::ToCross;
use crate::clay::{var::Cross, vm::runtime::Vm};

#[derive(Debug)]
struct Undef();

impl ToCross for Undef {}

pub fn new(vm:Vm)->Cross{
    Undef().to_cross(vm)
}

// pub fn test() {
//     let ud = undef();
//     let ud2 = ud.uncross();
//     let ud3 = ud2.cast::<Undef>();
//     println!("{:?}", ud3);
// }