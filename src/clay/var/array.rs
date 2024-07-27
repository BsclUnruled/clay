use std::{cell::RefCell,iter};
use super::Var;
use crate::clay::{var::ToVar, vm::{error, signal::{Abort, Signal}}};

pub type Array = RefCell<Vec<Var>>;

impl ToVar for Array{
    fn gc_iter(&self,this:&Var) -> Result<Box<dyn Iterator<Item = Signal>>, Abort>
        where Self:Sized + 'static {
        // let hc = self.borrow()
        //     .iter()
        //     .map(deref);
        // Box::new(hc)

        let this = this.clone();
        
        let mut conuter = 0;

        Ok(Box::new(iter::from_fn(move||{
            let hc = match this.unbox(){
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };

            let ori:&Array = match hc.cast(){
                Some(v) => v,
                None => return Some(Err(
                    error::use_dropped()
                )),
            };

            let index = conuter;
            conuter += 1;
            let x = ori.borrow().get(index).map(deref);
            x
        })))
    }
}

fn deref(a:&Var)->Signal{Ok(a.clone())}

pub fn new()->Array{
    RefCell::new(Vec::<Var>::new())
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