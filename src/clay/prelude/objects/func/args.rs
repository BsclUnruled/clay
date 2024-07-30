use std::rc::Rc;
use crate::clay::{var::{Var,VarBox}, vm:: runtime::Vm};

pub type Args<'l> = (
    Vm,
    &'l [Var],
    Rc<VarBox>,
    // &'l Yielder<Var, Signal>,
);