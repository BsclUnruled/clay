use std::rc::Rc;
use crate::clay::{var::VarBox, vm::{env, signal::{Abort, Signal}, Code}};
use super::Args;
use crate::clay::vm::Eval;

#[derive(Debug)]
pub struct Script{
    pub(super) name:String,
    args_names:Vec<String>,
    pub(super) code:Vec<Code>,
    ctx:Rc<VarBox>
}

impl Script{
    pub fn call(&self,arga4:Args)->Signal{
        let (vm,args,ctx) = arga4;
        // env::new_scope(||
        // })

        let ctx = env::default(
            vm,Some(Rc::clone(&ctx))
        );

        {
            for index in 0..(
                if self.args_names.len() > args.len(){args.len()}else{self.args_names.len()}
            ){
                let _ =ctx.def(vm, match &self.args_names.get(index){
                    Some(name)=>name,
                    None=>return Err(Abort::Throw(vm.undef()?))
                },&match args.get(index){
                    Some(arg)=>arg.clone(),
                    None=>return Err(Abort::Throw(vm.undef()?))
                });
                ()
            }         
            let mut result = vm.undef().into();
            for code in &self.code{
                result = code.eval(vm,Rc::clone(&self.ctx));
            }
            result
        }
    }

    pub fn new(
        name:&Option<String>,
        args_names:&[String],
        ctx:&Rc<VarBox>,
        code:&[Code]
    )->Self{
        let addr:() = ();
        Self{
            name:match name{
                Some(name)=>name.to_owned(),
                None=>format!("lambda@{:p}",&addr)
            },
            args_names:args_names.to_vec(),
            ctx:Rc::clone(ctx),
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