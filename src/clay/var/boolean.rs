use crate::clay::vm::error;

use super::{undef::undef, Var};

pub struct Bool(bool);

impl Bool {
    pub fn new(value: bool) -> Self {
        Self(value)
    }
}

impl Var for Bool {
    fn get(&self, name: &str) -> super::Cross {
        match name {
            _=>undef()
        }
    }
    fn set(&self, name: &str, _: super::Cross) {
        error::set_unsetable("Bool", name)
    }
}