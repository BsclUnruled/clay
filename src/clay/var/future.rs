use super::{undef::undef, Cross, ToCross};
use crate::clay::var::func::Args;
use crate::clay::vm::{
    signal::{Abort, Signal},
    Eval,
};
use std::mem;
use std::future::Future;
use std::pin::Pin;
use std::process::Output;

pub struct FutureWrapper(Pin<Box<dyn Future<Output = Cross>>>);

impl ToCross for FutureWrapper {}

impl FutureWrapper {
    pub fn new(future:Pin<Box<dyn Future<Output = Cross>>>) -> Self {
        FutureWrapper(future)
    }

    // pub fn resolve(args: Args) -> Signal {
    //     let (vm, args, ctx) = args;
    //     //vm.async_runtime().block_on()
    //     match args.get(0) {
    //         Some(future_code) => match future_code.eval(vm, ctx) {
    //             Ok(future_cross) => {
    //                 let hc = future_cross.uncross();
    //                 match hc.cast::<FutureWrapper>() {
    //                     Some(future) => Ok(vm.async_runtime().block_on(future.0)),
    //                     None => {
    //                         return Err(Abort::ThrowString(
    //                             "希望是一个Future对象(from Future.resolve)".to_owned(),
    //                         ))
    //                     }
    //                 }
    //             }
    //             Err(e) => return Err(e),
    //         },
    //         None => return Ok(undef()),
    //     }
    // }

}

// struct Fw<T: Future<Output = ()>>(T);

// const AF:

// async fn async_fn() -> () {}
