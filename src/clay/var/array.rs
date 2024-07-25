use std::cell::RefCell;
use super::Cross;
use crate::clay::var::ToCross;

pub type Array = RefCell<Vec<Cross>>;

impl ToCross for Array{}

pub fn new()->Array{
    RefCell::new(Vec::<Cross>::new())
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