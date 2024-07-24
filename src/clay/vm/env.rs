use std::cell::RefCell;
use std::collections::HashMap;

use crate::clay::var::Cross;
use std::rc::Rc;
use super::signal::Abort;
use super::signal::Signal;

pub trait Context {
    fn get(&self, name: &str)->Signal;
    fn set(&self, name: &str, value:&Cross);
    fn has(&self, name: &str)->bool;
    fn boxed(self:Self)->Box<dyn Context> where Self:Sized+'static{
        Box::new(self)
    }
    fn def(&self, name: &str, value:&Cross);
}

impl Context for (RefCell<HashMap<String, Cross>>,Option<Rc<dyn Context>>){
    fn get(&self, name: &str)->Signal {
        match self.0.borrow().get(name){
            Some(var) => Ok(var.clone()),
            None => match &self.1{
                Some(parent) => parent.get(name),
                None => Err(
                    Abort::ThrowString(
                        format!("Error(get):未能找到变量 {}",name)
                    )
                )
            }
        }
    }
    fn set(&self, name: &str, value:&Cross) {
        // self.0.borrow_mut().insert(name.to_string(), value.clone());
        if self.has(name){
            self.0.borrow_mut().insert(name.to_string(), value.clone());
        }else{
            match &self.1{
                Some(parent) => parent.set(name, value),
                None => panic!("Error(set):未能找到变量 {}",name)
            }
        }
    }
    fn has(&self, name: &str)->bool {
        self.0.borrow().contains_key(name)
    }
    fn def(&self, name: &str, value:&Cross){
        self.0.borrow_mut().insert(name.to_string(), value.clone());
    }
}

pub fn default(upper:Option<Rc<dyn Context>>)->Rc<dyn Context>{
    Rc::new((RefCell::new(HashMap::new()),upper))
}

// //自动创建并回收作用域
// pub fn new_scope(run:impl FnOnce()->Signal)->Signal{
//     CONTEXT.with(|ctx|{
//         ctx.borrow_mut()
//             .push_back(RefCell::new(HashMap::new()).boxed().into());
//     });
//     let result = run();
//     CONTEXT.with(|ctx|{
//         ctx.borrow_mut().pop_back();
//     });
//     result
// }

// //推入作用域,不回收
// pub fn use_scope(scope:Rc<dyn Context>, run:impl FnOnce()->Signal)->Signal{
//     CONTEXT.with(|ctx|{
//         ctx.borrow_mut().push_back(scope);
//     });
//     run()
// }

// pub fn find_var(name: &str)->Cross{
//     CONTEXT.with(|ctx|{
//         let c = ctx.borrow_mut().pop_back();
//         let result = match c{
//             Some(scope) => {
//                 let result = match scope.get(name){
//                     Some(var) => var.clone(),
//                     None => find_var(name)
//                 };
//                 ctx.borrow_mut().push_back(scope);
//                 result
//             },
//             None=>undef()
//         };
//         result
//     })
// }

// pub fn def_var(name: &str, value:Cross)->Signal{
//     CONTEXT.with(|ctx|{
//         // let result =  ctx.borrow().back().expect("Error(def_var):没有作用域了");
//         // result.set(name, value);

//         return match ctx.borrow_mut().back(){
//             Some(scope) => {
//                 scope.set(name,&value);
//                 Ok(value.clone())
//             },
//             None=>Err(
//                 Abort::ThrowString(
//                     "Error(def_var):没有作用域了".to_owned()
//                 )
//             ),
//         }
//     })
// }

// pub fn set_var(name: &str, value:Cross){
//     CONTEXT.with(|ctx|{
//         let c = ctx.borrow_mut().pop_back();
//         match c{
//             Some(scope) => {
//                 if scope.has(name){
//                     scope.set(name, &value);
//                 }else{
//                     set_var(name, value);
//                 };
//                 ctx.borrow_mut().push_back(scope);
//             },
//             None=>panic!("Error(set_var):没有作用域了")
//         };
//     });
// }