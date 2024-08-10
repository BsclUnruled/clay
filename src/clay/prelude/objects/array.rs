use std::cell::RefCell;
use crate::clay::var::Virtual;
use crate::clay::var::Var;


pub struct Array{
    inner: RefCell<Vec<Var>>,
}

impl Array{
    pub fn borrow(&self)->std::cell::Ref<'_, Vec<Var>>{
        self.inner.borrow()
    }

    pub fn borrow_mut(&self)->std::cell::RefMut<'_, Vec<Var>>{
        self.inner.borrow_mut()
    }

    pub fn new(v:Vec<Var>)->Self{
        Self{
            inner:RefCell::new(v),
        }
    }
}

impl Virtual for Array{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn gc_for_each(&self,marker:fn(&Var)){
        for x in self.borrow().iter(){
            marker(x)
        }
    }
}

pub fn new()->Array{
    Array::new(vec![])
}

// fn array_ctor(_:Args)->Signal{
//     let () = args;
//     let hc = RefCell::new(Vec::<Cross>::new());
//     hc.to_cross().into()
// }

// thread_local!{
//     static CTOR:Cross = {
//         let hc:Func = Func::Native(&array_ctor);
//         hc.to_cross()
//     }
// }

// pub fn ctor()->Cross{
//     CTOR.with(|f| f.clone())
// }