use std::{cell::RefCell, collections::HashMap};

use crate::clay::{var::{Var, Meta}, vm::{env::Env, promise::Promise, signal::Abort}};

pub struct Module {
    name: Option<String>,
    exports: RefCell<HashMap<String,Var>>
}

impl From<(String,HashMap<String,Var>)> for Module {
    fn from((name, exports): (String,HashMap<String,Var>)) -> Self {
        Module {
            name:Some(name),
            exports:RefCell::new(exports)
        }
    }
}

impl From<HashMap<String,Var>> for Module {
    fn from(exports: HashMap<String,Var>) -> Self {
        Module {
            name:None,
            exports:RefCell::new(exports)
        }
    }
}

impl Module{
    pub fn name(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("module@{:p}", self)
        }
    }
}

impl Meta for Module {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn def(&self,_:&Env, name: &str, value:&Var)->Promise {
        if self.exports.borrow().contains_key(name) {
            return 
                Err(
                    Abort::ThrowString(
                        format!("{} is already defined in module {}", name, self.name())
                    )
                ).into();
        }else{
            self.exports.borrow_mut().insert(name.to_string(), value.to_owned());
            Ok(value.clone()).into()
        }
    }

    fn gc_for_each(&self,f:fn(&Var)) {
        for (_, var) in self.exports.borrow().iter() {
            f(var);
        }
    }

    fn get(&self,env:&Env,name: &str)->Promise {
        if let Some(var) = self.exports.borrow().get(name) {
            Ok(var.clone()).into()
        }else{
            env.vm().undef().into()
        }
    }

    fn has(&self,_:&Env, name: &str)->bool {
        self.exports.borrow().contains_key(name)
    }

    fn set(&self,_:&Env, name: &str, value:&Var)->Promise{
        self.exports.borrow_mut().insert(name.into(), value.clone());
        Ok(value.clone()).into()
    }
}