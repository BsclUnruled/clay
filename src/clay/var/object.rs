use crate::clay::vm::keys;

use super::func::Args;
use super::{undef::undef, Cross, Var};
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Object {
    this:RefCell<HashMap<String,Cross>>
}

impl Object {
    pub fn new() -> Self {
        let mut core = HashMap::new();

        core.insert(keys::CLASS.to_string(),ctor());

        Object{
            this:RefCell::new(core)
        }
    }
    pub fn ctor(_:Args)->Cross{
        super::to_cross(Box::new(Object::new()))
    }
}

impl Var for Object {
    fn get(&self, name: &str) -> Cross {
        match self.this.borrow().get(name) {
            Some(v) => v.clone(),
            None => undef()
        }
    }
    fn set(&self, name: &str, value: Cross) {
        self.this
            .borrow_mut()
            .insert(name.to_string(), value.clone());
    }
}

thread_local! {
    static CTOR:Cross = super::func::new_ctor(&Object::ctor);
}

pub fn ctor() -> Cross {
    CTOR.with(|c| c.clone())
}