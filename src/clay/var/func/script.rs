use crate::clay::{var::{list, to_cross, undef::undef, Cross, Var}, vm::{env, error::set_unsetable, keys, Code}};
use super::{ctor, Args};
use crate::clay::vm::Eval;

#[derive(Debug)]
pub struct Script{
    args_name:Vec<String>,
    rest:Option<String>,
    pub(super) code:Vec<Code>,
}

impl Script{
    pub fn call(&self,args:Args)->Cross{
        env::new_scope(||{
            for index in 0..self.args_name.len(){
                env::def_var(&self.args_name[index],args.at(index).clone())
            }
            match &self.rest{
                Some(name)=>{
                    env::def_var(name,{
                        if args.args.len()<self.args_name.len(){
                            to_cross(Box::new(list::List::new(vec![])))
                        }else{
                            to_cross(Box::new(list::List::new(
                                args.args[self.args_name.len()..]
                                    .into_iter()
                                    .map(|code|{code.eval()})
                                    .collect()
                            )))
                        }
                    });
                },
                None=>()
            }
            let mut result = undef();
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

impl Var for Script{
    fn get(&self, name:&str)->Cross {
        match name{
            keys::CLASS=>ctor(),
            _=>undef()
        }
    }
    fn set(&self, name:&str, _:Cross) {
        set_unsetable("Func", name)
    }
}