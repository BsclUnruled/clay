use crate::clay::{prelude::objects::{args::Args, string}, var::ToVar};
use super::var::Var;
use signal::{Abort, Signal};
pub use runtime::Runtime;
pub mod env;
pub mod error;
pub mod gc;
pub mod runtime;
pub mod signal;
pub mod keys;


#[derive(Debug,Clone)]
pub enum Token {
    Id(String),

    // Int(BigInt),
    // Float(f64),

    Number(f64),

    Str(String),
    Template(String),

    Bracket(Vec<Token>),
    Large(Vec<Token>),
    Middle(Vec<Token>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),
}

// pub type CodeVec = Vec<Code>;

// #[derive(Debug, Clone)]
// pub enum Code {
//     Id(String),

//     Return(CodeVec),
//     Throw(CodeVec),
//     Break(CodeVec),
//     Continue(CodeVec),

//     If(CodeVec, CodeVec, CodeVec),
//     Try(CodeVec,CodeVec),

//     Op{
//         op:Operstor,
//         x:CodeVec,
//         y:CodeVec
//     },

//     Apply{
//         func:Box<Code>,
//         args:CodeVec,
//     },
//     Block(CodeVec),

//     Array(Vec<Code>),

//     Func{
//         name: Option<String>,
//         args_names: Vec<String>,
//         body: CodeVec,
//     },

//     Template(String),

//     //Option(String),
//     //Lambda(Vec<String>,Box<Code>),
//     The(Var),

//     //暂时保留
//     // Meta{//储存文件位置信息
//     //     file:Option<String>,
//     //     line:u32
//     // }
// }

// #[derive(Debug, Clone)]
// pub enum Operstor {
//     Add,Sub,Mul,Div,Mod,IntDiv,Pow,
//     Eq,Ne,Gt,Ge,Lt,Le,
//     And,Or,Not,
//     Neg,Index(CodeVec)
// }

pub type CtxType = Var;

pub trait Eval {
    fn eval(&self, all:Args) -> Signal;
}

impl Eval for Token{
    fn eval(&self, all:Args) -> Signal {
        use Token::*;
        match self {
            Id(name) => {
                all.ctx()
                   .unbox()?
                   .get(*all.vm(), &name)
            }
            Number(num) => Ok(num.to_var(*all.vm())),
            Str(s) => Ok(s.to_owned().to_var(*all.vm())),
            Template(s) => {
                let vm = all.vm();
                Ok(string::template(s, all.clone())?.to_var(*vm))
            },
            Bracket(tokens) => {
                match tokens.get(0){
                    _=>todo!()
                }
            },
            Large(tokens) => {
                let mut re = all.vm().undef()?;

                for token in tokens {
                    re = token.eval(all.clone())?;
                    all.ctx().unbox()?.set(*all.vm(), "it", &re)?;
                }

                Ok(re)
            },
            Middle(_)=>todo!(),
            The(var) => Ok(var.clone()),
        }
    }
}

impl Eval for String {
    fn eval(&self, all:Args) -> Signal {
        use crate::clay::parse::Parser;
        let parser = Parser::new(self);
        match parser.parse() {
            Ok(code) => code.eval(all),
            Err(e) => Err(Abort::ThrowString(e)),
        }
    }
}