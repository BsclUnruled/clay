use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::LinkedList;
use crate::clay::var::undef::undef;
use crate::clay::var::Cross;

use super::signal::Signal;

pub trait Context {
    fn get(&self, name: &str)->Option<Cross>;
    fn set(&self, name: &str, value: Cross);
    fn has(&self, name: &str)->bool;
    fn boxed(self:Self)->Box<dyn Context> where Self:Sized+'static{
        Box::new(self)
    }
}

impl Context for RefCell<HashMap<String, Cross>>{
    fn get(&self, name: &str)->Option<Cross> {
        match self.borrow().get(name){
            Some(var) => Some(var.clone()),
            None => None
        }
    }
    fn set(&self, name: &str, value: Cross) {
        self.borrow_mut().insert(name.to_string(), value);
    }
    fn has(&self, name: &str)->bool {
        self.borrow().contains_key(name)
    }
}

thread_local! {
    static CONTEXT:RefCell<LinkedList<Box<dyn Context>>> = RefCell::new(LinkedList::new());
}

pub fn new_scope(run:impl FnOnce()->Signal)->Signal{
    CONTEXT.with(|ctx|{
        ctx.borrow_mut()
            .push_back(Box::new(RefCell::new(HashMap::new())));
    });
    let result = run();
    CONTEXT.with(|ctx|{
        ctx.borrow_mut().pop_back();
    });
    result
}

pub fn find_var(name: &str)->Cross{
    CONTEXT.with(|ctx|{
        let c = ctx.borrow_mut().pop_back();
        let result = match c{
            Some(scope) => {
                let result = match scope.get(name){
                    Some(var) => var.clone(),
                    None => find_var(name)
                };
                ctx.borrow_mut().push_back(scope);
                result
            },
            None=>undef()
        };
        result
    })
}

pub fn def_var(name: &str, value: Cross){
    CONTEXT.with(|ctx|{
        ctx.borrow()
           .back()
           .expect("Error(def_var):没有作用域了")
           .set(name, value);
    });
}

pub fn set_var(name: &str, value: Cross){
    CONTEXT.with(|ctx|{
        let c = ctx.borrow_mut().pop_back();
        match c{
            Some(scope) => {
                if scope.has(name){
                    scope.set(name, value);
                }else{
                    set_var(name, value);
                };
                ctx.borrow_mut().push_back(scope);
            },
            None=>panic!("Error(set_var):没有作用域了")
        };
    });
}