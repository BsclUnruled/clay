use std::cell::RefCell;
use super::{func::Args,Cross};
use crate::clay::var::ToCross;
use crate::clay::var::func::*;
use crate::clay::vm::signal::Signal;

pub type Array = RefCell<Vec<Cross>>;

pub fn new()->Array{
    RefCell::new(Vec::<Cross>::new())
}

fn array_ctor(_:Args)->Signal{
    let hc = RefCell::new(Vec::<Cross>::new());
    hc.to_cross().into()
}

thread_local!{
    static CTOR:Cross = {
        let hc:Func = Func::Native(&array_ctor);
        hc.to_cross()
    }
}

pub fn ctor()->Cross{
    CTOR.with(|f| f.clone())
}