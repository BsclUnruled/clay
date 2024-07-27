use crate::clay::{var::func::Args, vm::signal::Signal};

pub fn debug(all:Args)->Signal{
    let (vm, _,_) = all;
    print!("debug");
    vm.borrow().undef()
}