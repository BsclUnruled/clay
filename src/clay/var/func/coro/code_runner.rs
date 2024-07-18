use crate::clay::{var::{func::Script, undef::undef, Cross, VarBox}, vm::Code};
use std::rc::Rc;
use std::cell::Cell;

use super::{Coro, Stage};

pub enum CodeRunner{
    Code(Rc<VarBox>,Cell<usize>),
    Result(Cross)
}

impl CodeRunner{
    pub fn new(code:Cross)->Self{
        match code.uncross().cast::<Script>() {
            Some(_) => {
                Self::Code(code.uncross(),Cell::new(0))
            }
            None => {
                panic!("Error:试图")
            }
        }
    }
    pub fn run(&self,coro:&Coro)->Signal{
        match coro.stage.get() {
            Stage::Finished=>Signal::End(undef()),
            _=>match self {
                Self::Result(result)=>{
                    coro.stage.set(Stage::Finished);
                    Signal::End(result.clone())
                },
                Self::Code(code,index)=>{
                    let bind = code.clone();
                    let code:Option<&Script> = bind.cast();
                    match code {
                        Some(code)=>{
                            let bytecode = &code.code[index.get()];
                            
                            if index.get()>=code.code.len(){
                                coro.stage.set(Stage::Finished);
                            };

                            let signal = match bytecode {
                                Code::Block(ref func,ref args)=>{
                                    
                                },
                                _=>panic!("VmError:未知字节码")
                            };
                            index.set(index.get()+1);
                            signal
                        }
                        None=>panic!("Error:试图")
                    }
                }
            }
        }
    }
    pub fn result(result:Cross)->Self{
        Self::Result(result)
    }
}

pub enum Signal{
    Return(Cross),
    End(Cross),
    Throw(Cross),
    Break(Cross),
    Yield(Cross),
}