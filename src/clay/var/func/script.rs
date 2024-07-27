use std::rc::Rc;
use crate::clay::{var::{array::Array, ToVar}, vm::{env, signal::{Abort, Signal}, Code}};
use super::Args;
use crate::clay::vm::Eval;

#[derive(Debug)]
pub struct Script{
    pub(super) name:String,
    args_name:Vec<String>,
    rest:Option<String>,
    pub(super) code:Vec<Code>,
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
                    Some(arg)=>arg.clone(),
                    None=>return Err(Abort::Throw(vm.borrow().undef()?))
                });
                ()
            }
            match &self.rest{
                Some(name)=>{
                    ctx.def(name,{
                        &(if args.len()<self.args_name.len(){
                            Array::new(vec![]).to_var(vm)
                        }else{
                            Array::new(
                                // build((vm,&args[self.args_name.len()..],Rc::clone(&ctx)))?
                                {
                                    let args = &args[self.args_name.len()..];
                                    let hc = args.into_iter()
                                        .fold(Ok(Vec::with_capacity(args.len())),|acc: Result<Vec<crate::clay::var::Var>, Abort>,cross|{
                                            match acc{
                                                Ok(mut acc_vec)=>{
                                                    acc_vec.push(cross.clone());
                                                    Ok(acc_vec)
                                                }
                                                Err(e)=>return Err(e)
                                            }
                                    });
                                    hc
                                }?
                            ).to_var(vm)
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

    pub fn new(name:Option<String>,args_name:Vec<String>, rest:Option<String>, code:&[Code])->Self{
        let addr:() = ();
        Self{
            name:match name{
                Some(name)=>name,
                None=>format!("lambda@{:?}",&addr as *const ())
            },
            args_name,
            rest,
            code:code.to_vec(),
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