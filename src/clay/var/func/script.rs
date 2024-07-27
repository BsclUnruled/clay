use std::rc::Rc;
use crate::clay::{var::{array::Array, ToVar}, vm::{env, signal::{Abort, Signal}, Token}};
use super::Args;
use crate::clay::vm::Eval;

#[derive(Debug)]
pub struct Script{
    pub(super) name:String,
    args_name:Vec<String>,
    rest:Option<String>,
    pub(super) code:Vec<Token>,
}

impl Script{
    pub fn call(&self,arga4:Args)->Signal{
        let (vm,args,ctx) = arga4;
        // env::new_scope(||
        // })

        let ctx = env::default(
            Some(Rc::clone(&ctx))
        );

        {
            for index in 0..(
                if self.args_name.len() > args.len(){args.len()}else{self.args_name.len()}
            ){
                ctx.def( match &self.args_name.get(index){
                    Some(name)=>name,
                    None=>return Err(Abort::Throw(vm.borrow().undef()?))
                },&match args.get(index){
                    Some(arg)=>arg,
                    None=>return Err(Abort::Throw(vm.borrow().undef()?))
                }.eval(vm,Rc::clone(&ctx))?);
                ()
            }
            match &self.rest{
                Some(name)=>{
                    ctx.def(name,{
                        &(if args.len()<self.args_name.len(){
                            Array::new(vec![]).to_cross(vm)
                        }else{
                            Array::new(
                                // build((vm,&args[self.args_name.len()..],Rc::clone(&ctx)))?
                                {
                                    let args = &args[self.args_name.len()..];
                                    let hc = args.into_iter()
                                        .fold(Ok(Vec::with_capacity(args.len())),|acc,arg|{
                                            match acc{
                                                Ok(mut acc_vec)=>{
                                                    match arg.eval(vm,Rc::clone(&ctx)){
                                                        Ok(cross)=>Ok({acc_vec.push(cross);acc_vec}),
                                                        Err(e)=>return Err(e)
                                                    }
                                                }
                                                Err(e)=>return Err(e)
                                            }
                                    });
                                    hc
                                }?
                            ).to_cross(vm)
                        })
                    });
                },
                None=>()
            }
            let mut result = vm.borrow().undef().into();
            for code in &self.code{
                result = code.eval(vm,Rc::clone(&ctx));
            }
            result
        }
    }

    pub fn new(name:Option<String>,args_name:Vec<String>, rest:Option<String>, code:Vec<Token>)->Self{
        let addr:() = ();
        Self{
            name:match name{
                Some(name)=>name,
                None=>format!("lambda@{:?}",&addr as *const ())
            },
            args_name,
            rest,
            code,
        }
    }
}

// fn build(args:Args)->Result<Vec<Cross>,Abort>{
//     let (vm,args,ctx,_) = args;
//     let hc = args.into_iter()
//         .fold(Ok(Vec::with_capacity(args.len())),|acc,arg|{
//             match acc{
//                 Ok(mut acc_vec)=>{
//                     match arg.eval(vm,Rc::clone(&ctx)){
//                         Ok(cross)=>Ok({acc_vec.push(cross);acc_vec}),
//                         Err(e)=>return Err(e)
//                     }
//                 }
//                 Err(e)=>return Err(e)
//             }
//     });
//     hc
// }