use crate::clay::{var::{array::{self, Array},undef::undef, Cross}, vm::{env, signal::{Abort, Signal}, Code}};
use super::Args;
use crate::clay::var::ToCross;

#[derive(Debug)]
pub struct Script{
    args_name:Vec<String>,
    rest:Option<String>,
    pub(super) code:Vec<Code>,
}

impl Script{
    pub fn call(&self,args:Args)->Signal{
        env::new_scope(||{
            for index in 0..(
                if self.args_name.len() > args.len(){args.len()}else{self.args_name.len()}
            ){
                env::def_var(&self.args_name[index],args[index].eval()?)
            }
            match &self.rest{
                Some(name)=>{
                    env::def_var(name,{
                        if args.len()<self.args_name.len(){
                            Array::new(vec![]).to_cross()
                        }else{
                            array::Array::new(
                                build(&args[self.args_name.len()..])?
                            ).to_cross()
                        }
                    });
                },
                None=>()
            }
            let mut result = undef().into();
            for code in &self.code{
                result = code.eval();
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
    let hc = args.into_iter()
        .fold(Ok(Vec::with_capacity(args.len())),|acc,arg|{
            match acc{
                Ok(mut acc_vec)=>{
                    match arg.eval(){
                        Ok(cross)=>Ok({acc_vec.push(cross);acc_vec}),
                        Err(e)=>return Err(e)
                    }
                }
                Err(e)=>return Err(e)
            }
    });
    hc
}