use std::{iter, rc::Rc};
use crate::clay::{var::{Var, Virtual}, vm::{env, signal::{Abort, Signal}, Code, CtxType, Eval}};
use super::{args::Args, native::Function};
use crate::clay::vm::runtime::Exit;

pub struct Script{
    pub(super) name:String,
    pub(super) args_names:Vec<String>,
    pub(super) code:Vec<Code>,
    pub(super) ctx:CtxType
}

impl Script{
    pub fn cons_ctx(&self,all:&Args)->CtxType{
        let vm = *all.vm();

        let undef = match vm.undef(){
            Ok(undef)=>undef.clone(),
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
                (name.to_owned(),arg.clone())
            })
            .collect();

        env::from_map(map,Some(Rc::clone(&self.ctx)))
    }

    pub fn call(&self,all:Args)->Signal{
        let vm = all.vm();
        let args = all.args();

        let ctx = env::default(
            all.ctx()
        );

        {
            for index in 0..(
                if self.args_names.len() > args.len(){args.len()}else{self.args_names.len()}
            ){
                let _ = ctx.def(*vm, match &self.args_names.get(index){
                    Some(name)=>name,
                    None=>return Err(Abort::Throw(vm.undef()?))
                },&match args.get(index){
                    Some(arg)=>arg.clone(),
                    None=>return Err(Abort::Throw(vm.undef()?))
                });
            }         
            let mut result = vm.undef().into();
            for code in &self.code{
                result = code.eval(Args::from(
                    (*vm,&[] as &[Var],Rc::clone(&self.ctx))
                ));
            }
            result
        }
    }

    pub fn new(
        name:&Option<String>,
        args_names:&[String],
        ctx:&CtxType,
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