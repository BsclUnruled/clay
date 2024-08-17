use std::iter;
use crate::clay::{var::Virtual, vm::{env, signal::{Abort, Signal}, CtxType, Eval, Token}};
use super::{args::Args, native::Function};
use crate::clay::vm::runtime::Exit;

pub struct Script{
    pub(super) name:String,
    pub(super) args_names:Vec<String>,
    pub(super) code:Vec<Token>,
    pub(super) ctx:CtxType
}

impl Script{
    pub fn cons_ctx(&self,all:&Args)->CtxType{
        let vm = *all.vm();

        let undef = match vm.undef(){
            Ok(undef)=>Token::The(undef.clone()),
            Err(e)=>vm.exit(e)
        };

        let iter = all.args().iter().chain(
            iter::from_fn(||{
                Some(&undef)
            })
        );

        let map = self
            .args_names
            .iter()
            .zip(iter)
            .map(|(name,arg)|{
                (name.to_owned(),arg.eval(all.clone()))
            })
            .map(|(name,signal)|{
                let hc = match signal{
                    Ok(var)=>var,
                    Err(e)=>vm.exit(e)
                };
                (name,hc)
            })
            .collect();

        env::from_map(vm,map,Some(self.ctx.clone()))
    }

    pub fn call(&self,all:Args)->Signal{
        let vm = all.vm();
        let args = all.args();

        let ctx = env::default(
            *vm,
            self.ctx.clone()
        );

        {
            for index in 0..(
                if self.args_names.len() > args.len(){args.len()}else{self.args_names.len()}
            ){
                let _ = all.ctx().unbox()?.def(*vm, match &self.args_names.get(index){
                    Some(name)=>name,
                    None=>return Err(Abort::Throw(vm.undef()?))
                },&match args.get(index){
                    Some(arg)=>arg.eval(
                        Args::from(
                            (*vm,&[] as &[Token],all.ctx().clone())
                        )
                    )?,
                    None=>return Err(Abort::Throw(vm.undef()?))
                });
            }         
            let mut result = vm.undef().into();
            for code in &self.code{
                result = code.eval(Args::from(
                    (*vm,&[] as &[Token],ctx.clone())
                ));
            }
            result
        }
    }

    pub fn new(
        name:&Option<String>,
        args_names:&[String],
        ctx:&CtxType,
        code:&[Token]
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

impl Virtual for Func{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn call(&self,all:Args)->Signal {
        match self {
            Self::Script(script)=>script.call(all),
            Self::Native(native)=>native.call(all)
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