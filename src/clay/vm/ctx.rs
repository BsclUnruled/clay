use crate::clay::var::ToVar;
use crate::clay::var::Meta;
use crate::clay::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use crate::clay::var::Var;
use super::env::Env;
use super::error;
use super::promise::resolve;
use super::promise::Promise;
use super::runtime::Vm;
use super::signal::Abort;
use super::CtxType;

// pub trait Context{
//     fn get(&self,vm:Vm, name: &str)->Promise;

//     fn set(&self,vm:Vm, name: &str, value:&Var)->Promise;

//     fn has(&self,_:Vm, name: &str)->bool;

//     fn def(&self,_:Vm , name: &str, value:&Var)->Promise;

//     fn for_each(&self,_:fn(&Var)) {}
// }

pub struct Ctx(pub(crate)Cell<HashMap<String, Var>>,pub(crate) CtxType);

// impl Debug for Ctx{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Context")
//     }
// }

// impl Display for Ctx{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Context")
//     }
// }

impl Meta for Ctx{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get(&self,env:&Env, name: &str)->Promise {
        match self.0.borrow().get(name){
            Some(var) => Ok(var.clone()).into(),
            None => self.1.get(env,name)
        }
    }
    fn set(&self,vm:&Env, name: &str, value:&Var)->Promise{
        // self.0.borrow_mut().insert(name.to_string(), value.clone());
        if self.has(vm,name){
            self.0.borrow_mut().insert(name.to_string(), value.clone());
            resolve(Ok(value.clone()))
        }else{
            self.1.get(vm,name)
        }
    }

    fn has(&self,_:&Env, name: &str)->bool {
        self.0.borrow().contains_key(name)
    }

    fn def(&self,_:&Env , name: &str, value:&Var)->Promise{
        self.0.borrow_mut().insert(name.to_string(), value.clone());
        Ok(value.clone()).into()
    }

    fn gc_for_each(&self,f:fn(&Var)) {
        for (_, var) in self.0.borrow().iter() {
            f(var);
        }
    }
}

pub fn from_map(vm:Vm,map:HashMap<String, Var>,upper:Option<CtxType>) -> CtxType{
    Ctx(
        Cell::new(map),
        match upper {
            Some(upper) => upper,
            None => void_ctx(vm)
        }
    ).to_var(vm)
}

pub fn default(vm:Vm,upper:CtxType)->CtxType{
    Ctx(
        Cell::new(HashMap::new()),
        upper
    ).to_var(vm)
}

#[derive(Debug)]
pub struct Void();

impl Display for Void{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Void")
    }
}

impl Meta for Void{
    fn gc_for_each(&self,_:fn(&Var)) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn def(&self,env:&Env, name: &str, _:&Var)->Promise{
        Err(error::throw(env.vm(),&format!("Error(def {:?} to undef_ctx):没有作用域了 (from undef_ctx)",name))).into()
    }
    fn get(&self,_:&Env, name: &str)->Promise {
        Err(
            Abort::ThrowString(
                format!("Error(get {:?} from undef_ctx):没有作用域了",name)
            )
        ).into()
    }
    fn has(&self,_:&Env, _: &str)->bool {
        false
    }
    fn set(&self,env:&Env, name: &str, _:&Var)->Promise{
        Err(error::throw(env.vm(),&format!("Error(set {:?} to undef_ctx):没有作用域了 (from undef_ctx)",name)))
            .into()
    }
}

pub fn void_ctx(vm:Vm) -> CtxType{
    Void().to_var(vm)
}