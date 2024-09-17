use std::iter;
use crate::clay::{var::{Var, Meta}, vm::{ctx, env::Env, promise::{resolve, Promise}, CtxType, Eval, ToRun}};
use super::native::Function;
use crate::clay::vm::runtime::Exit;

pub struct Script{
    pub(super) name:String,
    pub(super) args_names:Vec<String>,
    pub(super) code:Vec<ToRun>,
    pub(super) ctx:CtxType
}

impl Script{
    pub fn cons_ctx(&self,env:&Env,args:&[Var])->CtxType{
        let vm = *env.vm();

        let undef = match vm.undef(){
            Ok(undef)=>undef.clone(),
            Err(e)=>vm.exit(e)
        };

        let iter = args.iter().chain(
            iter::from_fn(||{
                Some(&undef)
            })
        );

        let map = self
            .args_names
            .iter()
            .zip(iter)
            .map(|(name,arg)|{
                (name.to_owned(),arg.clone())
            })
            .collect();

        ctx::from_map(vm,map,Some(self.ctx.clone()))
    }

    pub fn call(&self,env:&Env,args:&[Var])->Promise{
        let vm = env.vm();

        {
            for index in 0..(
                if self.args_names.len() > args.len(){args.len()}else{self.args_names.len()}
            ){
                let _ = env.ctx().def(env, match &self.args_names.get(index){
                    Some(name)=>name,
                    None=>env.vm().exit(&format!("Invalid argument index: {}",index) as &str)
                },&match args.get(index){
                    Some(arg)=>arg.clone(),
                    None=>env.vm().exit(&format!("Invalid argument index: {}",index) as &str)
                });
            }         
            let mut result = vm.undef();
            for code in &self.code{
                result = code.eval(env,&[]);
            }
            resolve(result)
        }
    }

    pub fn new(
        name:&Option<String>,
        args_names:&[String],
        ctx:&CtxType,
        code:&[ToRun]
    )->Self{
        let addr:() = ();
        Self{
            name:match name{
                Some(name)=>name.to_owned(),
                None=>format!("lambda@{:p}",&addr)
            },
            args_names:args_names.to_vec(),
            ctx:ctx.clone(),
            code:code.to_vec(),
        }
    }
}

pub enum Func{
    Script(Script),
    Native(super::native::Native)
}

impl From<Function> for  Func{
    fn from(value: Function) -> Self {
        Func::Native(
            super::native::Native::new(
                value,
                None
            )
        )
    }
}

impl From<(Function,String)> for Func{
    fn from((value,name):(Function,String)) -> Self {
        Func::Native(
            super::native::Native::new(
                value,
                Some(name)
            )
        )
    }
}

impl Meta for Func{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn call(&self,env:&Env,args:&[Var])->Promise{
        match self {
            Self::Script(script)=>script.call(env,args),
            Self::Native(native)=>native.call(env,args)
        }
    }
}

impl Func{
    pub fn name(&self)->String{
        match self {
            Self::Script(script)=>script.name.to_owned(),
            Self::Native(native)=>native.name()
        }
    }
}