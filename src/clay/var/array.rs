use std::{cell::RefCell, fmt::Display, iter};
use super::{func::Args, Var};
use crate::clay::{var::Virtual, vm::{error, signal::{Abort, Signal}}};

#[derive(Debug)]
pub struct Array{
    inner: RefCell<Vec<Var>>,
}

impl Display for Array{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[")?;
        
        match self.borrow().get(0){
            Some(v) => Display::fmt(v,f),
            None => Ok(()),
        }?;

        for x in self.borrow().iter(){
            write!(f,", ")?;

            match x.unbox(){
                Ok(v) => Ok(Display::fmt(&**v,f)),
                Err(_) => Err(std::fmt::Error{}),
            }??;
        }
        write!(f,"]")?;
        Ok(())
    }
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
    fn as_func(&self,_:Args)->Signal
    where Self:Sized + 'static{
        Err(
            error::not_a_func()
        )
    }

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