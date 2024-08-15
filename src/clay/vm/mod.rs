use crate::clay::{parse::t2c, prelude::objects::args::Args};
use super::var::{Var, VarBox};
use signal::{Abort, Signal};
use std::rc::Rc;
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
    Line(Vec<Token>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),
}

pub type CodeVec = Vec<Code>;

#[derive(Debug, Clone)]
pub enum Code {
    Id(String),

    Return(CodeVec),
    Throw(CodeVec),
    Break(CodeVec),
    Continue(CodeVec),

    If(CodeVec, CodeVec, CodeVec),
    Try(CodeVec,CodeVec),

    Op{
        op:Operstor,
        x:CodeVec,
        y:CodeVec
    },

    Apply{
        func:Box<Code>,
        args:CodeVec,
    },
    Block(CodeVec),

    Array(Vec<Code>),

    Func{
        name: Option<String>,
        args_names: Vec<String>,
        body: CodeVec,
    },

    Template(String),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),

    //暂时保留
    // Meta{//储存文件位置信息
    //     file:Option<String>,
    //     line:u32
    // }
}

#[derive(Debug, Clone)]
pub enum Operstor {
    Add,Sub,Mul,Div,Mod,IntDiv,Pow,
    Eq,Ne,Gt,Ge,Lt,Le,
    And,Or,Not,
    Neg,Index(CodeVec)
}

pub type CtxType = Rc<VarBox>;

pub trait Eval {
    fn eval(&self, all:Args) -> Signal;
}

impl Eval for String {
    fn eval(&self, all:Args) -> Signal {
        use crate::clay::parse::Parser;
        let parser = Parser::new(self);
        match parser.parse() {
            Ok(code) => t2c(&code,all.vm())?.eval(all.clone()),
            Err(e) => Err(Abort::ThrowString(e)),
        }
    }
}

impl Eval for Code {
    fn eval(&self, _all:Args) -> Signal {
        todo!()
    }
}

impl Eval for [Code] {
    fn eval(&self, _all:Args) -> Signal {
        todo!()
    }
}
