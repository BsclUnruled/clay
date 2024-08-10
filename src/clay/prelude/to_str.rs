
use crate::clay::{var::ToVar, vm::runtime::Vm};

use super::objects::method:: Method;

pub fn global_init(vm:Vm){
    let _ = vm.get_context().def(vm,"str",
        &Method::new("str".to_owned().into()).to_var(vm)
    );
}

// fn basic(all:Args)->Signal{
//     let obj = &all.args()[0];
//     Ok(
//         format!("Object@{}",obj.unbox()?.ptr()).to_var(*all.vm())
//     )
// }

// fn show_str(all:Args)->Signal{
//     match all.args().get(0){
//         Some(s)=>Ok(s.clone()),
//         None=>Err(
//             Abort::ThrowString(format!("str参数不足"))
//         )
//     }
// }

// fn show_int(all:Args)->Signal{
//     match all.args().get(0){
//         Some(i)=>Ok(i.unbox()?.cast::<BigInt>()?.to_string().to_var(*all.vm())),
//         None=>Err(
//             Abort::ThrowString(format!("str参数不足"))
//         )
//     }
// }