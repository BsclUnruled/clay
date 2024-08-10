use std::{cell::RefCell, collections::HashMap};

use crate::clay::{var::{Var, Virtual}, vm::{env::Context, signal::Abort}};

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

impl Context for Module {
    fn def(&self,_:crate::clay::vm::runtime::Vm , name: &str, value:&Var)->crate::clay::vm::signal::Signal {
        if self.exports.borrow().contains_key(name) {
            return 
                Err(
                    Abort::ThrowString(
                        format!("{} is already defined in module {}", name, self.name())
                    )
                );
        }else{
            self.exports.borrow_mut().insert(name.to_string(), value.clone());
            Ok(value.clone())
        }
    }

    fn for_each(&self,f:fn(&Var)) {
        for (_, var) in self.exports.borrow().iter() {
            f(var);
        }
    }

    fn get(&self,vm:crate::clay::vm::runtime::Vm, name: &str)->crate::clay::vm::signal::Signal {
        if let Some(var) = self.exports.borrow().get(name) {
            Ok(var.clone())
        }else{
            vm.undef()
        }
    }

    fn has(&self,_:crate::clay::vm::runtime::Vm, name: &str)->bool {
        self.exports.borrow().contains_key(name)
    }

    fn set(&self,_:crate::clay::vm::runtime::Vm, name: &str, value:&Var)->crate::clay::vm::signal::Signal {
        self.exports.borrow_mut().insert(name.into(), value.clone());
        Ok(value.clone())
    }
}

impl Virtual for Module {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}