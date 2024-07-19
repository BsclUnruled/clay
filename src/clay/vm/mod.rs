use num::BigInt;
use signal::Signal;
use crate::clay::var::ToCross;
use super::var::{func::Func, string, undef::undef};
pub mod gc;
pub mod error;
pub mod keys;
pub mod env;
pub mod signal;

#[derive(Debug,Clone)]
pub enum Code{
    Sym(String),
    Int(BigInt),
    Float(f64),
    Str(String),
    Escape(String),
    Template(String),
    Bracket(Box<Code>,Vec<Code>),
    Block(Vec<Code>),
}

impl Code{
    pub fn eval(&self)->Signal{
        match self{
            Self::Sym(ref s)=>env::find_var(s),
            Self::Int(ref i)=>i.clone().to_cross(),
            Self::Float(ref f)=>(*f).to_cross(),
            Self::Str(ref s)=>s.to_owned().to_cross(),
            Self::Escape(ref s)=>string::escape(s).to_cross(),
            Self::Template(ref s)=>string::template(s).to_cross(),
            Self::Block(ref b)=>{
                return env::new_scope(||{
                    let mut result = undef().into();
                    for expr in b{
                        result = expr.eval();
                    };
                    result
                })
            },
            Self::Bracket(ref b,ref args)=>{
                let hc = b.eval()?.uncross();
                let func:&Func = match hc.cast(){
                    Some(f)=>f,
                    None=>panic!("不是函数")
                };
                func.call(args)?
            }
        }.into()
    }
}