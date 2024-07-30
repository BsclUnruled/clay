use crate::clay::vm::signal::{Abort, Signal};
use super::func::Args;

fn make(_:Args)->Signal{
    Err(Abort::ThrowString("Error:Lambda尚未完成".to_owned()))
}

// thread_local! {
//     static MAKE:Cross = Func::Native(&make).to_cross();
// }

// pub fn lambda()->Cross{
//     MAKE.with(|f| f.clone())
// }