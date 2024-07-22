use env::Context;
use num_bigint::BigInt;
use signal::{Abort, Signal};
use crate::clay::var::ToCross;
use super::var::{func::Func, string, undef::undef, Cross};
pub mod gc;
pub mod error;
pub mod keys;
pub mod env;
pub mod signal;
pub mod runtime;

pub use runtime::Runtime;

#[derive(Debug,Clone)]
pub enum Code{
    Sym(String),

    Int(BigInt),
    Float(f64),

    Str(String),
    Template(String),

    Bracket(Vec<Code>),
    Block(Vec<Code>),
    Middle(Vec<Code>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Cross)
}

impl ToString for Code{
    fn to_string(&self)->String{
        match self{
            Self::Block(ref b)=>{
                format!("{{\n{:?}\n}}",b.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(", "))
            }
            Self::Bracket(ref b)=>{
                format!("({})",b.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(", "))
            }
            Self::Middle(ref b)=>{
                format!("[{}]",b.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(", "))
            }
            Self::Sym(ref s)=>format!("Sym({})",s),
            Self::Int(ref i)=>format!("Int({})",i),
            Self::Float(ref f)=>format!("Float({})",f),
            Self::Str(ref s)=>format!("\"{}\"",s),
            Self::Template(ref s)=>format!("`{}`",s),
            Self::The(_)=>"*Cross实值*".to_owned()
        }
    }
}

pub trait Eval {
    fn eval(&self,vm:&mut Runtime,ctx:&dyn Context)->Signal;
}

impl Eval for String{
    fn eval(&self,vm:&mut Runtime,ctx:&dyn Context)->Signal{
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

impl Eval for [Code]{
    fn eval(&self,vm:&mut Runtime,ctx:&dyn Context)->Signal{
        match self.get(0){
            None=>undef().into(),
            Some(func_sym)=>{
                match func_sym.eval(vm,ctx){
                    Ok(func)=>{
                        match func.uncross().cast::<Func>(){
                            Some(f)=>f.call((vm,&self[1..],ctx)),
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

impl Eval for Code{
    fn eval(&self,vm:&mut Runtime,ctx:&dyn Context)->Signal{
        match self{
            Self::Sym(ref s)=>env::find_var(s),
            Self::Int(ref i)=>i.clone().to_cross(),
            Self::Float(ref f)=>(*f).to_cross(),
            Self::Str(ref s)=>s.to_owned().to_cross(),
            Self::Template(ref s)=>string::template(s).to_cross(),
            Self::Block(ref b)=>{
                return env::new_scope(||{
                    let mut result = undef().into();
                    for expr in b{
                        result = expr.eval(vm,ctx);
                    };
                    result
                })
            },
            Self::Bracket(ref args)=>{
                let hc = match args.get(0){
                    Some(fun)=>fun,
                    None=>return Err(
                        Abort::Throw(
                            undef()
                        )
                    )
                }.eval(vm,ctx)?.uncross();
                let func:&Func = match hc.cast(){
                    Some(f)=>f,
                    None=>return Err(
                        Abort::ThrowString(
                            "不是函数(from Eval for Code::Bracket)".to_owned()
                        )
                    )
                };
                func.call((vm,&args[1..],ctx))?
            },
            Self::Middle(ref args)=>{
                let hc = match args.get(0){
                    Some(fun)=>fun,
                    None=>return Err(
                        Abort::Throw(
                            undef()
                        )
                    )
                }.eval(vm,ctx)?.uncross();
                let func:&Func = match hc.cast(){
                    Some(f)=>f,
                    None=>return Err(
                        Abort::ThrowString(
                            "不是函数(from Eval for Code::Middle)".to_owned()
                        )
                    )
                };
                func.call((vm,&args[1..],ctx))?
            }
            Self::The(ref c)=>c.clone(),
        }.into()
    }
}