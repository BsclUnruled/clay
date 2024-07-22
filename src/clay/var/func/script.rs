use crate::clay::{var::{array::{self, Array},undef::undef, Cross}, vm::{env, signal::{Abort, Signal}, Code}};
use super::Args;
use crate::clay::var::ToCross;
use crate::clay::vm::Eval;

#[derive(Debug)]
pub struct Script{
    args_name:Vec<String>,
    rest:Option<String>,
    pub(super) code:Vec<Code>,
}

impl Script{
    pub fn call(&self,arga4:Args)->Signal{
        let (vm,args,ctx) = arga4;
        env::new_scope(||{
            for index in 0..(
                if self.args_name.len() > args.len(){args.len()}else{self.args_name.len()}
            ){
                env::def_var( match &self.args_name.get(index){
                    Some(name)=>name,
                    None=>return Err(Abort::Throw(undef()))
                },match args.get(index){
                    Some(arg)=>arg,
                    None=>return Err(Abort::Throw(undef()))
                }.eval(vm,ctx)?)?;
                ()
            }
            match &self.rest{
                Some(name)=>{
                    env::def_var(name,{
                        if args.len()<self.args_name.len(){
                            Array::new(vec![]).to_cross()
                        }else{
                            array::Array::new(
                                build((vm,&args[self.args_name.len()..],ctx))?
                            ).to_cross()
                        }
                    })?;
                },
                None=>()
            }
            let mut result = undef().into();
            for code in &self.code{
                result = code.eval(vm,ctx);
            }
            result
        })
    }

    pub fn new(args_name:Vec<String>, rest:Option<String>, code:Vec<Code>)->Self{
        Self{
            args_name,
            rest,
            code,
        }
    }
}

fn build(args:Args)->Result<Vec<Cross>,Abort>{
    let (vm,args,ctx) = args;
    let hc = args.into_iter()
        .fold(Ok(Vec::with_capacity(args.len())),|acc,arg|{
            match acc{
                Ok(mut acc_vec)=>{
                    match arg.eval(vm,ctx){
                        Ok(cross)=>Ok({acc_vec.push(cross);acc_vec}),
                        Err(e)=>return Err(e)
                    }
                }
                Err(e)=>return Err(e)
            }
    });
    hc
}