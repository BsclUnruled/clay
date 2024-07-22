use super::{undef::undef, Cross, ToCross};
use crate::clay::var::func::Args;
use crate::clay::vm::{
    signal::{Abort, Signal},
    Eval,
};
use std::mem;
use std::future::Future;

pub struct FutureWrapper<T: Future<Output = Cross> + Send + 'static>(T);

impl<T: Future<Output = Cross> + Send + 'static> ToCross for FutureWrapper<T> {}

impl<T: Future<Output = Cross> + Send + 'static> FutureWrapper<T> {
    pub fn new(future: T) -> Self {
        FutureWrapper(future)
    }

    // pub fn resolve(args: Args) -> Signal {
    //     let (vm, args, ctx) = args;
    //     //vm.async_runtime().block_on()
    //     match args.get(0) {
    //         Some(future_code) => match future_code.eval(vm, ctx) {
    //             Ok(future_cross) => {
    //                 let hc = future_cross.uncross();
    //                 match hc.cast::<FutureWrapper<T>>() {
    //                     Some(future) => Ok(vm.async_runtime().block_on(async{
    //                         mem::replace(&mut future.0,async{undef()}).await
    //                     })),
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
