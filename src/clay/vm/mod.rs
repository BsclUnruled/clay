use crate::clay::prelude::objects::args::Args;
use super::var::{ToVar, Var};
use num_bigint::BigInt;
use runtime::Vm;
use signal::{Abort, ErrSignal, Signal};
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

    Int(BigInt),
    Float(f64),

    Str(String),
    Template(String),

    Bracket(Vec<Token>),
    Block(Vec<Token>),
    Middle(Vec<Token>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),
}

// impl Eval for Token {
//     fn eval(&self, vm: Vm, ctx:&CtxType) -> Signal {
//         Ok(match self {
//             Token::Id(ref s) => ctx.get(vm, s)?,
//             Token::Int(ref i) => i.to_owned().to_var(vm),
//             Token::Float(ref f) => f.to_var(vm),
//             Token::Str(ref s) => s.to_owned().to_var(vm),
//             Token::Template(ref s) => string::template(s, ctx).to_var(vm),
//             Token::Bracket(ref b) => {
//                 //b.len()>=1
//                 debug_assert!(b.len() >= 1);

//                 if b.len() == 1 {//直接求值
//                     b[0].eval(vm, ctx)?
//                 }else{
//                     let func = b[0].eval(vm, &ctx)?;
//                     func.unbox()?.call(Args::new(vm,&{
//                         let mut hc = Vec::with_capacity(b.len() - 1);
//                         for token in b.iter().skip(1) {
//                             hc.push(token.eval(vm, &ctx)?);
//                         }
//                         hc
//                     }))?
//                 }
//             }
//             Token::Block(ref b) => {
//                 let new_ctx = env::default(Some(Rc::clone(&ctx)));
//                 let mut result = vm.undef()?;
//                 for token in b {
//                     result = token.eval(vm, &new_ctx)?;
//                 };
//                 result
//             }
//             Token::Middle(_)=>todo!("下标: 开发中"),
//             Token::The(ref c) => c.clone(),
//         })
//     }
// }

impl Token {
    pub fn format(&self,vm:Vm) -> ErrSignal<Code> {
        match self {
            Token::Id(ref s) => Ok(Code::Id(s.clone())),

            Token::Int(ref i) => Ok(Code::The(i.to_owned().to_var(vm))),
            Token::Float(ref f) => Ok(Code::The(f.to_var(vm))),

            Token::Str(ref s) => Ok(Code::The(s.to_owned().to_var(vm))),
            Token::Template(ref s) => Ok(Code::Template(s.clone())),

            // Token::Bracket(ref b) => Code::Bracket(b.iter().map(|t| t.format()).collect()),
            // Token::Block(ref b) => Code::Block(b.iter().map(|t| t.format()).collect()),
            // Token::Middle(ref b) => Code::Middle(b.iter().map(|t| t.format()).collect()),

            Token::Block(ref b)=>{
                Ok(Code::Block({
                    let mut hc = Vec::with_capacity(b.len());
                    for code in b {
                        hc.push(code.format(vm)?);
                    }
                    hc
                }))
            }

            Token::Middle(_)=>
                Err(Abort::ThrowString(
                    format!("unexpected token: middle")
                )),

            Token::Bracket(ref expr)=>{
                //todo!("重头戏")

                let mut bracket = vec![];
                let mut stage1 = vec![];

                for token in expr.iter(){
                    if let Token::Id(ref s) = token {
                        match s.as_str() {
                            "+" | "-" | "*" | "/" | "++" | "--" | "%" |
                            "==" | "!=" | ">=" | "<=" | ">" | "<" |
                            "&&" | "||" | "!" | "neg" | "@" | ":" | "~"
                            =>{
                                stage1.push(bracket);
                                stage1.push(vec![token]);

                                bracket = vec![];
                            }
                            _=>bracket.push(token)
                        }
                    }else{
                        bracket.push(token)
                    }
                };

                drop(bracket);

                let _stage1_iter = stage1.into_iter().map(|tv|(tv,0usize));

                todo!("重头戏: 对运算符进行提升");

                //()
            }

            Token::The(ref c) => Ok(Code::The(c.clone())),
        }
    }
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

pub type CtxType = Rc<dyn env::Context>;

pub trait Eval {
    fn eval(&self, all:Args) -> Signal;
}

impl Eval for String {
    fn eval(&self, all:Args) -> Signal {
        use crate::clay::parse::Parser;
        let parser = Parser::new(self);
        match parser.parse() {
            Ok(code) => code.format(*all.vm())?.eval(all.clone()),
            Err(e) => Err(Abort::ThrowString(e)),
        }
    }
}

impl Eval for Code {
    fn eval(&self, _all:Args) -> Signal {
        todo!()
    }
}
