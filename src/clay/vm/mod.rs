use crate::clay::var::Cross;
use super::var::{float::*, func::{Args, Func}, int::*, string, to_cross, undef::undef};
pub mod gc;
pub mod error;
pub mod keys;
pub mod env;

#[derive(Debug)]
pub enum Code{
    Sym(String),
    Int(BigInt),
    Float(f64),
    Str(String),
    Escape(String),
    Template(String),
    Bracket(Vec<Code>),
    Block(Vec<Vec<Code>>),
}

pub trait Eval{
    fn eval(&self)->Cross;
}

impl Eval for Code{
    fn eval(&self)->Cross{
        match self{
            Self::Sym(ref s)=>env::find_var(s),
            Self::Int(ref i)=>to_cross(Box::new(i.clone())),
            Self::Float(ref f)=>to_cross(Box::new(*f)),
            Self::Str(ref s)=>to_cross(Box::new(s.to_string())),
            Self::Escape(ref s)=>to_cross(Box::new(string::escape(s))),
            Self::Template(ref s)=>to_cross(Box::new(string::template(s))),
            Self::Block(ref b)=>{
                env::new_scope(||{
                    let mut result = undef();
                    for expr in b{
                        result = expr.eval();
                    };
                    result
                })
            },
            Self::Bracket(ref b)=>b.eval(),
            _=>panic!("未知Code")
        }
    }
}

impl Eval for Vec<Code>{
    fn eval(&self)->Cross {
        let func = match self.first(){
            Some(ref tbf) => tbf.eval(),
            None=>return undef()
        };
        let args = Args{args:self[1..].to_vec()};
        let func:&Func = match func.uncross().cast(){
            Some(f)=>f,
            None=>return undef()
        };
        func.call(args)
    }
}