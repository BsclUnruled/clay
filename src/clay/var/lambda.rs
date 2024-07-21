use crate::clay::vm::signal::{Abort, Signal};
use crate::clay::var::Cross;
use super::func::Args;
use crate::clay::var::func::Func;
use crate::clay::var::ToCross;

fn make(_:Args)->Signal{
    Err(Abort::ThrowString("Error:Lambda尚未完成".to_owned()))
}

thread_local! {
    static MAKE:Cross = Func::Native(&make).to_cross();
}

pub fn lambda()->Cross{
    MAKE.with(|f| f.clone())
}