use env::Context;
use std::rc::Rc;
use num_bigint::BigInt;
use runtime::Vm;
use signal::{Abort, Signal};
use crate::clay::var::ToVar;
use super::var::{func::Func, string,Var};
pub mod gc;
pub mod error;
pub mod keys;
pub mod env;
pub mod signal;
pub mod runtime;

pub use runtime::Runtime;

#[derive(Debug,Clone)]
pub enum Token{
    Ident(String),

    Int(BigInt),
    Float(f64),

    Str(String),
    Template(String),

    Bracket(Vec<Token>),
    Block(Vec<Token>),
    Middle(Vec<Token>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var)
}

// impl ToString for Code{
//     fn to_string(&self)->String{
//         match self{
//             Self::Block(ref b)=>{
//                 format!("{{\n{:?}\n}}",b.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(", "))
//             }
//             Self::Bracket(ref b)=>{
//                 format!("({})",b.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(", "))
//             }
//             Self::Middle(ref b)=>{
//                 format!("[{}]",b.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(", "))
//             }
//             Self::Sym(ref s)=>format!("Sym({})",s),
//             Self::Int(ref i)=>format!("Int({})",i),
//             Self::Float(ref f)=>format!("Float({})",f),
//             Self::Str(ref s)=>format!("\"{}\"",s),
//             Self::Template(ref s)=>format!("`{}`",s),
//             Self::The(_)=>"*Cross实值*".to_owned()
//         }
//     }
// }

pub trait Eval {
    fn eval(&self,vm:Vm,ctx:Rc<dyn Context>)->Signal;
}

impl Eval for String{
    fn eval(&self,vm:Vm,ctx:Rc<dyn Context>)->Signal{
        use crate::clay::parse::Parser;
        let parser = Parser::new(self);
        match parser.parse(){
            Ok(code)=>code.eval(vm,ctx),
            Err(e)=>Err(
                Abort::ThrowString(e)
            )
        }
    }
}

impl Eval for [Token]{
    fn eval(&self,vm:Vm,ctx:Rc<dyn Context>)->Signal{
        match self.get(0){
            None=>vm.borrow().undef().into(),
            Some(func_sym)=>{
                match func_sym.eval(vm,Rc::clone(&ctx)){
                    Ok(func)=>{
                        match func.unbox()?.cast::<Func>(){
                            Some(f)=>f.call((vm,&self[1..],Rc::clone(&ctx))),
                            None=>Err(
                                Abort::ThrowString(
                                    "不是函数(from Eval for [Code])".to_owned()
                                )
                            )
                        }
                    }
                    Err(e)=>Err(e)
                }
            }
        }
    }
}

impl Eval for Token{
    fn eval(&self,vm:Vm,ctx:Rc<dyn Context>)->Signal{
        match self{
            Self::Ident(ref s)=>ctx.get(s)?,
            Self::Int(ref i)=>i.clone().to_cross(vm),
            Self::Float(ref f)=>(*f).to_cross(vm),
            Self::Str(ref s)=>s.to_owned().to_cross(vm),
            Self::Template(ref s)=>string::template(s,ctx).to_cross(vm),
            Self::Block(ref b)=>{
                let new_ctx = env::default(
                    Some(Rc::clone(&ctx))
                );

                let mut result = vm.borrow().undef()?;

                for line in b{
                    result = line.eval(vm,Rc::clone(&new_ctx))?;
                }
                result
            },
            Self::Bracket(ref args)=>{
                if args.len() == 1{
                    match args.get(0){
                        Some(fun)=>fun,
                        None=>return Err(
                            // Abort::ThrowString(
                            //     format!("Error: 变量已回收(from Eval for Code::Bracket)")
                            // )
                            error::use_dropped()
                        )
                    }.eval(vm,Rc::clone(&ctx))?
                }else{
                    let hc = match args.get(0){
                        Some(fun)=>fun,
                        None=>return Err(
                            Abort::ThrowString(
                                format!("Error: 变量已回收(from Eval for Code::Bracket)")
                            )
                        )
                    }.eval(vm,Rc::clone(&ctx))?.unbox()?;
                    let func:&Func = match hc.cast(){
                        Some(f)=>f,
                        None=>return Err(
                            Abort::ThrowString(
                                "不是函数(from Eval for Code::Bracket)".to_owned()
                            )
                        )
                    };
                    func.call((vm,&args[1..],Rc::clone(&ctx)))?
                }
            },
            Self::Middle(ref args)=>{
                let hc = match args.get(0){
                    Some(fun)=>fun,
                    None=>return Err(
                        Abort::Throw(
                            vm.borrow().undef()?
                        )
                    )
                }.eval(vm,Rc::clone(&ctx))?.unbox()?;

                #[cfg(debug_assertions)]{
                    println!("尝试执行函数")
                }

                let func:&Func = match hc.cast(){
                    Some(f)=>f,
                    None=>return Err(
                        Abort::ThrowString(
                            "不是函数(from Eval for Code::Middle)".to_owned()
                        )
                    )
                };
                func.call((vm,&args[1..],Rc::clone(&ctx)))?
            }
            Self::The(ref c)=>c.clone(),
        }.into()
    }
}