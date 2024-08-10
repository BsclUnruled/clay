use crate::clay::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use crate::clay::var::Var;
use std::rc::Rc;
use super::error;
use super::runtime::Vm;
use super::signal::Abort;
use super::signal::Signal;
use super::CtxType;

pub trait Context{
    fn get(&self,vm:Vm, name: &str)->Signal;

    fn set(&self,vm:Vm, name: &str, value:&Var)->Signal;

    fn has(&self,_:Vm, name: &str)->bool;

    fn def(&self,_:Vm , name: &str, value:&Var)->Signal;

    fn for_each(&self,_:fn(&Var)) {}
}

pub struct Ctx(pub(crate)Cell<HashMap<String, Var>>,pub(crate)Option<CtxType>);

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

impl Context for Ctx{
    fn get(&self,vm:Vm, name: &str)->Signal {
        match self.0.borrow().get(name){
            Some(var) => Ok(var.clone()),
            None => match &self.1{
                Some(parent) => parent.get(vm,name),
                None => Err(
                    Abort::ThrowString(
                        format!("Error(get):未能找到变量 {:?}",name)
                    )
                )
            }
        }
    }
    fn set(&self,vm:Vm, name: &str, value:&Var)->Signal{
        // self.0.borrow_mut().insert(name.to_string(), value.clone());
        if self.has(vm,name){
            self.0.borrow_mut().insert(name.to_string(), value.clone());
            Ok(value.clone())
        }else{
            match &self.1{
                Some(parent) => parent.set(vm,name, value),
                None => panic!("Error(set):未能找到变量 {:?}",name)
            }
        }
    }

    fn has(&self,_:Vm, name: &str)->bool {
        self.0.borrow().contains_key(name)
    }

    fn def(&self,_:Vm , name: &str, value:&Var)->Signal{
        self.0.borrow_mut().insert(name.to_string(), value.clone());
        Ok(value.clone())
    }

    fn for_each(&self,f:fn(&Var)) {
        for (_, var) in self.0.borrow().iter() {
            f(var);
        }
    }
}

pub fn from_map(map:HashMap<String, Var>,upper:Option<CtxType>) -> CtxType{
    Rc::new(Ctx(
        Cell::new(map),
        upper
    ))
}

pub fn default(upper:Option<CtxType>)->CtxType{
    Rc::new(Ctx(
        Cell::new(HashMap::new()),
        upper
    ))
}

#[derive(Debug)]
pub struct Void();

impl Display for Void{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Void")
    }
}

impl Context for Void{
    fn for_each(&self,_:fn(&Var)) {}

    fn def(&self,vm:Vm, name: &str, _:&Var)->Signal{
        Err(error::throw(vm,&format!("Error(def {:?} to undef_ctx):没有作用域了 (from undef_ctx)",name)))
    }
    fn get(&self,_:Vm, name: &str)->Signal {
        Err(
            Abort::ThrowString(
                format!("Error(get {:?} from undef_ctx):没有作用域了",name)
            )
        )
    }
    fn has(&self,_:Vm, _: &str)->bool {
        false
    }
    fn set(&self,vm:Vm, name: &str, _:&Var)->Signal{
        Err(error::throw(vm,&format!("Error(set {:?} to undef_ctx):没有作用域了 (from undef_ctx)",name)))
    }
}

pub fn void_ctx() -> CtxType{
    Rc::new(Void())
}

// //自动创建并回收作用域
// pub fn new_scope(run:impl FnOnce()->Signal)->Signal{
//     CONTEXT.with(|ctx|{
//         ctx.borrow_mut()
//             .push_back(Cell::new(HashMap::new()).boxed().into());
//     });
//     let result = run();
//     CONTEXT.with(|ctx|{
//         ctx.borrow_mut().pop_back();
//     });
//     result
// }

// //推入作用域,不回收
// pub fn use_scope(scope:Rc<VarBox>, run:impl FnOnce()->Signal)->Signal{
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