use crate::clay::vm::error;
use std::cell::RefCell;
use super::{func::Args, undef::undef, Cross, Var, VarBox};
use std::rc::Rc;

struct List{
    pub(crate) data:RefCell<Vec<Cross>>
}

impl List{
    fn new(v:Vec<Cross>)->Self{
        Self{
            data:RefCell::new(v)
        }
    }
    fn ctor(args:&Args)->Cross{
        let v = args.get_args().clone();
        Rc::new(
            VarBox::new(Box::new(List::new(v)))
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