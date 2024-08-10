use num_bigint::BigInt;
use crate::clay::prelude::objects::args::Args;
use crate::clay::var::ToVar;
use crate::clay:: vm::signal::{Abort, Signal};

pub fn add(all:Args)->Signal{
    let vm = all.vm();
    let args = all.args();

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
    let x_int:&BigInt = binding.cast()?;
    let binding = y.unbox()?;
    let y_int:&BigInt = binding.cast()?;

    let result = x_int + y_int;

    Ok(result.to_var(*vm))
}