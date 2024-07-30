use crate::clay::var::ToVar;
use crate::clay::var::VarBox;
use crate::clay::var::Virtual;
use crate::clay::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use crate::clay::var::Var;
use std::rc::Rc;
use super::error;
use super::gc::Mark;
use super::runtime::Vm;
use super::signal::Abort;
use super::signal::ErrSignal;
use super::signal::Signal;

pub struct Context(pub(crate)Cell<HashMap<String, Var>>,pub(crate)Option<Rc<VarBox>>);

impl Debug for Context{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context")
    }
}

impl Display for Context{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context")
    }
}

impl Virtual for Context{
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
    fn set(&self,vm:Vm, name: &str, value:&Var)->ErrSignal<()>{
        // self.0.borrow_mut().insert(name.to_string(), value.clone());
        if self.has(vm,name){
            self.0.borrow_mut().insert(name.to_string(), value.clone());
            Ok(())
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

    fn def(&self,_:Vm , name: &str, value:&Var)->ErrSignal<()>{
        self.0.borrow_mut().insert(name.to_string(), value.clone());
        Ok(())
    }
}

pub fn default(vm:Vm,upper:Option<Rc<VarBox>>)->Rc<VarBox>{
    Context(Cell::new(HashMap::new()),upper).to_var(vm).unbox().unwrap()
}

#[derive(Debug)]
pub struct Void();

impl Display for Void{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Void")
    }
}

impl Virtual for Void{
    fn def(&self,_:Vm, name: &str, _:&Var)->ErrSignal<()>{
        Err(error::throw(&format!("Error(def {:?} to undef_ctx):没有作用域了 (from undef_ctx)",name)))
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
    fn set(&self,_:Vm, name: &str, _:&Var)->ErrSignal<()>{
        Err(error::throw(&format!("Error(set {:?} to undef_ctx):没有作用域了 (from undef_ctx)",name)))
    }
}

pub fn void_ctx() -> Rc<VarBox>{
    Rc::new(VarBox{
        mark:std::cell::Cell::new(Mark::New),
        value:Box::new(Void()),
        id:0,
    })
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