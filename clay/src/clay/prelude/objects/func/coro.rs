use std::{cell::{Cell, RefCell}, rc::Rc};

use super::{Args, Script};

use corosensei::{Coroutine, CoroutineResult};

use crate::clay::{var::{undef::undef, Cross}, vm::{env::Context, signal::{Abort, Signal}, Code, Eval, Runtime}};

pub struct Coro{
    inner:RefCell<Coroutine<Cross,Signal,Signal>>,
    // args_name:Vec<String>,
    // rest:Option<String>,
    // pub(super) code:Vec<Code>,
    context:Rc<dyn Context>,
}

impl Coro {
    pub fn new(
        vm:&'static RefCell<Runtime>,
        func:&Script,
        context:Rc<dyn Context>,)->Self{
            
        let cont1xt = Rc::clone(&context);//给结构体
        let cont2xt = Rc::clone(&cont1xt);//给协程

        let code = func.code.clone();

        Self{
            inner:RefCell::new(Coroutine::new(move|yielder,val|{
                //todo:初始化context
                //

                yielder.suspend(Ok(undef()));//先暂停

                let mut result = Ok(undef());
                for code in code{
                    result = code.eval(vm,Rc::clone(&cont2xt));
                }
                result
            })),
            context:cont1xt
        }
    }
    pub fn resume(&self,args:Args)->Signal{
        if !self.done(){
            let (vm,args,ctx) = args;

            let final_args = match args.get(0){
                Some(arg) => arg.eval(vm,Rc::clone(&ctx))?,
                None => undef()
            };

            match self.inner.borrow_mut().resume(final_args){
                CoroutineResult::Return(signal) => signal,
                CoroutineResult::Yield(signal) => signal,
            }
        }else{
            Ok(
                undef()
            )
        }
    }
    pub fn done(&self)->bool{
        self.inner.borrow().done()
    }
}