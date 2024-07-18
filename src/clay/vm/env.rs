use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::LinkedList;
use crate::clay::var::undef::undef;
use crate::clay::var::Cross;

thread_local! {
    static CONTEXT:RefCell<LinkedList<RefCell<HashMap<String,Cross>>>> = RefCell::new(LinkedList::new());
}

pub fn new_scope(run:impl FnOnce()->Cross)->Cross{
    CONTEXT.with(|ctx|{
        ctx.borrow_mut()
            .push_back(RefCell::new(HashMap::new()));
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
                let result = match scope.borrow_mut().get(name){
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
           .borrow_mut()
           .insert(name.to_string(), value);
    });
}

pub fn set_var(name: &str, value: Cross){
    CONTEXT.with(|ctx|{
        let c = ctx.borrow_mut().pop_back();
        match c{
            Some(scope) => {
                if scope.borrow().contains_key(name){
                    scope.borrow_mut().insert(name.to_string(), value);
                }else{
                    set_var(name, value);
                };
                ctx.borrow_mut().push_back(scope);
            },
            None=>panic!("Error(set_var):没有作用域了")
        };
    });
}