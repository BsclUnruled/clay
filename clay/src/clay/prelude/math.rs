use num_bigint::BigInt;
use crate::clay::prelude::objects::func::args::Args;
use crate::clay::var::ToVar;
use crate::clay:: vm::signal::{Abort, Signal};

pub fn add(all:Args)->Signal{
    let (vm,args,_) = all;
    let x = match args.get(0) {
        Some(x)=>x,
        None=>return Err(
            Abort::ThrowString("X呢?".to_owned())
        )
    };

    let y = match args.get(1) {
        Some(y)=>y,
        None=>return Err(
            Abort::ThrowString("Y呢?".to_owned())
        )
    };

    let binding = x.unbox()?;
    let x_o:Option<&BigInt> = binding.cast();
    let binding = y.unbox()?;
    let y_o:Option<&BigInt> = binding.cast();

    let x_int = match x_o {
        Some(x_int)=>x_int,
        None=>return Err(
            Abort::ThrowString("X不是整数".to_owned())
        )
    };

    let y_int = match y_o {
        Some(y_int)=>y_int,
        None=>return Err(
            Abort::ThrowString("Y不是整数".to_owned())
        )
    };

    let result = x_int + y_int;

    Ok(result.to_var(vm))
}