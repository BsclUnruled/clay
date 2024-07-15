use std::rc::Rc;
use std::ops::Deref;
use std::cell::Cell;

use super::vm;
use super::vm::gc::Mark;

pub mod list;
pub mod undef;

pub trait Var{
    fn get(&self, name:&str)->Cross;
    fn set(&self, name:&str, value:Cross);
}

pub fn to_cross<V>(value:V)->Cross where V : Var{
    Rc::new(VarBox::new(value))
}

pub struct VarBox{
    mark:Cell<Mark>,
    id:usize,
    value:Box<dyn Var>,
}

impl VarBox{
    pub fn new<V>(value:V)->Self where V : Var{
        Self{
            mark:Cell::new(Mark::New),
            id:vm::gc::get_id(),
            value:Box::new(value),
        }
    }

    pub fn get_id(&self)->usize{self.id}
    pub fn get_super(&self)->Cross{
        self.value.get("--super--")
    }
    pub fn get_class(&self)->Cross{
        self.value.get("--class--")
    }
    pub fn get_mark(&self)->Mark{self.mark.get()}
    pub fn set_mark(&self, mark:Mark){self.mark.set(mark)}
}

impl Deref for VarBox{
    type Target = dyn Var;
    fn deref(&self)->&Self::Target{&*self.value}
}

pub type Cross = Rc<VarBox>;