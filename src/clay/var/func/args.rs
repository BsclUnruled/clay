use crate::clay::var::{to_cross, undef};
use crate::clay::vm::error::set_unsetable;
use crate::clay::vm::{keys, Code};
use crate::clay::vm::Eval;
use super::super::Var;

use super::super::Cross;

pub struct Args {
    pub(crate) args: Vec<Code>,
}

impl Args {
    pub fn new(args: Vec<Code>) -> Self {
        Self { args }
    }
    pub fn ctor(_: Args) -> Cross {
        to_cross(Box::new(Self {
            args: vec![],
        }))
    }
    pub fn at(&self, index: usize) -> Cross {
        if self.args.len() > index {
            self.args[index].eval()
        } else {
            undef()
        }
    }
}

impl Var for Args {
    fn get(&self, name: &str) -> Cross {
        match name {
            keys::CLASS => ARGS_CTOR.with(|ctor| ctor.clone()),
            _ => undef(),
        }
    }
    fn set(&self, name: &str, _: Cross) {
        set_unsetable("Args", name)
    }
}

thread_local!{
    static ARGS_CTOR:Cross = to_cross(super::Native::new(&Args::ctor));
}