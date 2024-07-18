use crate::clay::vm::error;
use std::cell::RefCell;
use super::{func::Args, undef::undef, Cross, Var};
use crate::clay::vm::Eval;

pub struct List{
    pub(crate) data:RefCell<Vec<Cross>>
}

impl List{
    pub fn new(v:Vec<Cross>)->Self{
        Self{
            data:RefCell::new(v)
        }
    }
    pub fn ctor(args:Args)->Cross{
        //解构Arg

        let v:Vec<Cross> = args.args.into_iter().map(|code|{
            code.eval()
        }).collect();
        Cross::new(
            Box::new(List::new(v))
        )
    }
}

thread_local!{
    static CTOR:Cross = super::func::new_ctor(&List::ctor);
}

pub fn ctor()->Cross{
    CTOR.with(|f| f.clone())
}

impl Var for List{
    fn get(&self, name:&str)->Cross {
        match name{
            _=>undef()
        }
    }
    fn set(&self, name:&str, _:Cross) {
        error::set_unsetable("List", name)
    }
}