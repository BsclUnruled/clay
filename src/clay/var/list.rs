use crate::clay::vm::error;
use std::cell::RefCell;
use super::{undef::undef, Cross, Var};

struct List{
    data:RefCell<Vec<Cross>>
}

impl Var for List{
    fn get(&self, name:&str)->Cross {
        match name{
            _=>undef()
        }
    }
    fn set(&self, name:&str, value:Cross) {
        error::set_unsetable("List", name)
    }
}