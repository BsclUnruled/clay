use crate::clay::prelude::objects::args::Args;
use crate::clay:: vm::signal::Signal;
use crate::clay::vm::{Eval, Token};

pub fn add(all:Args)->Signal{
    let vm = *all.vm();
    let args = all.args();

    let mut hc_vec = Vec::with_capacity(args.len());

    for arg in args {
        let hc = arg.eval(all.clone())?;
        hc_vec.push(hc);
    }

    let mut sum = vm.undef()?;
    let mut flag = true;

    for hc in hc_vec {
        if flag {
            sum = hc;
            flag = false;
        }else{
            sum = sum.unbox()?
                .get(vm, "#add")?
                .unbox()?
                .call(Args::new(vm, &[Token::The(hc)],all.ctx().clone()))?
        }
    }

    Ok(sum)
}