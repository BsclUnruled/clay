use super::var::Var;
use env::Env;
use signal::{Abort, Signal};
pub use runtime::Runtime;
pub mod ctx;
pub mod env;
pub mod error;
pub mod runtime;
pub mod signal;
pub mod keys;
pub mod promise;
pub mod heap;

pub type ToRun = Ast;

type AstVec = Box<[Ast]>;

#[derive(Debug, Clone)]
pub enum Ast {
    Id(String),
    Template(String),
    Str(String),
    Number(f64),

    None,//没有任何操作

    Return(AstVec),
    Throw(AstVec),
    Break(AstVec),
    Continue(AstVec),

    If(AstVec, AstVec, AstVec),
    Try(AstVec,AstVec),

    Op{
        op:Operstor,
        x:AstVec,
        y:AstVec
    },

    TryApply{
        func:Box<Ast>,
        args:AstVec,
    },
    Block(AstVec),

    Array(AstVec),

    Func{
        name: Option<String>,
        args_names: Box<[String]>,
        body: AstVec,
    },

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
    Neg
}

impl ToString for Operstor {
    fn to_string(&self) -> String {
        use Operstor::*;
        match self {
            Add => "+".to_string(),
            Sub => "-".to_string(),
            Mul => "*".to_string(),
            Div => "/".to_string(),
            Mod => "%".to_string(),
            IntDiv => "//".to_string(),
            Pow => "**".to_string(),
            Eq => "==".to_string(),
            Ne => "!=".to_string(),
            Gt => ">".to_string(),
            Ge => ">=".to_string(),
            Lt => "<".to_string(),
            Le => "<=".to_string(),
            And => "&&".to_string(),
            Or => "||".to_string(),
            Not => "!".to_string(),
            Neg => "neg".to_string(),
        }
    }
}

pub type CtxType = Var;

pub trait Eval {
    fn eval(&self,env:&Env,args:&[Var]) -> Signal;
}

impl Eval for String {
    fn eval(&self,env:&Env,args:&[Var]) -> Signal {
        use crate::clay::parse::parse;
        let result = parse(self);
        match result {
            Ok(code) => code.eval(env,args),
            Err(e) => Err(Abort::ThrowString(e.to_string())),
        }
    }
}

impl Eval for Ast {
    fn eval(&self,_env:&Env,_args:&[Var]) -> Signal {
        todo!()
    }
}