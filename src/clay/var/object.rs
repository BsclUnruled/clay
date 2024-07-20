use crate::clay::vm::keys;
use crate::clay::vm::signal::Signal;

use super::func::Args;
use super::ToCross;
use super::{undef::undef, Cross};
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Object {
    this:RefCell<HashMap<String,Cross>>
}

impl ToCross for Object {
    fn to_cross(self) -> Cross {
        Cross::new(
            Box::new(self)
        )
    }
}

impl Object {
    pub fn new() -> Self {
        let mut core = HashMap::new();

        //core.insert(keys::CLASS.to_string(),ctor());
        core.insert(keys::SUPER.to_string(),undef());

        Object{
            this:RefCell::new(core)
        }
    }
    pub fn ctor(_:Args)->Signal{
        Object::new().to_cross().into()
    }
}

// thread_local! {
//     static CTOR:Cross = super::func::new_ctor(&Object::ctor);
// }

// pub fn ctor() -> Cross {
//     CTOR.with(|c| c.clone())
// }