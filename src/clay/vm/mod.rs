use super::var::{string, ToVar, Var};
use env::Context;
use num_bigint::BigInt;
use runtime::Vm;
use signal::{Abort, Signal};
use std::rc::Rc;
pub use runtime::Runtime;

pub mod env;
pub mod error;
pub mod gc;
pub mod keys;
pub mod runtime;
pub mod signal;


#[derive(Debug, Clone)]
pub enum Token {
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
    The(Var),
}

impl Token {
    pub fn format(&self) -> Code {
        match self {
            Token::Ident(ref s) => Code::Ident(s.clone()),

            Token::Int(ref i) => Code::Int(i.clone()),
            Token::Float(ref f) => Code::Float(*f),

            Token::Str(ref s) => Code::Str(s.clone()),
            Token::Template(ref s) => Code::Template(s.clone()),

            Token::Bracket(ref b) => Code::Bracket(b.iter().map(|t| t.format()).collect()),
            Token::Block(ref b) => Code::Block(b.iter().map(|t| t.format()).collect()),
            Token::Middle(ref b) => Code::Middle(b.iter().map(|t| t.format()).collect()),

            Token::The(ref c) => Code::The(c.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Code {
    Ident(String),

    Int(BigInt),
    Float(f64),

    Str(String),
    Template(String),

    Bracket(Vec<Code>),
    Block(Vec<Code>),
    Middle(Vec<Code>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),
}

pub trait Eval {
    fn eval(&self, vm: Vm, ctx: Rc<dyn Context>) -> Signal;
}

impl Eval for String {
    fn eval(&self, vm: Vm, ctx: Rc<dyn Context>) -> Signal {
        use crate::clay::parse::Parser;
        let parser = Parser::new(self);
        match parser.parse() {
            Ok(code) => code.format().eval(vm, ctx),
            Err(e) => Err(Abort::ThrowString(e)),
        }
    }
}

impl Eval for [Code] {
    fn eval(&self, vm: Vm, ctx: Rc<dyn Context>) -> Signal {
        match self.get(0) {
            None => vm.borrow().undef(),
            Some(func_sym) => match func_sym.eval(vm, Rc::clone(&ctx)) {
                Ok(var) => {
                    let hc = {
                        let mut args: Vec<Var> = Vec::with_capacity(self.len() - 1);
                        for code in self.iter().skip(1) {
                            args.push(code.eval(vm, Rc::clone(&ctx))?);
                        }
                        args
                    };
                    var.unbox()?.as_func((vm, &hc, Rc::clone(&ctx)))
                }
                Err(e) => Err(e),
            },
        }
    }
}

impl Eval for Code {
    fn eval(&self, vm: Vm, ctx: Rc<dyn Context>) -> Signal {
        match self {
            Self::Ident(ref s) => ctx.get(s)?,
            Self::Int(ref i) => i.clone().to_var(vm),
            Self::Float(ref f) => (*f).to_var(vm),
            Self::Str(ref s) => s.to_owned().to_var(vm),
            Self::Template(ref s) => string::template(s, ctx).to_var(vm),
            Self::Block(ref b) => {
                let new_ctx = env::default(Some(Rc::clone(&ctx)));

                let mut result = vm.borrow().undef()?;

                for line in b {
                    result = line.eval(vm, Rc::clone(&new_ctx))?;
                }
                result
            }
            Self::Bracket(ref args) => {
                if args.len() == 1 {
                    match args.get(0) {
                        Some(fun) => fun,
                        None => {
                            return Err(
                                // Abort::ThrowString(
                                //     format!("Error: 变量已回收(from Eval for Code::Bracket)")
                                // )
                                error::use_dropped(),
                            );
                        }
                    }
                    .eval(vm, Rc::clone(&ctx))?
                } else {
                    let var = match args.get(0) {
                        Some(fun) => fun,
                        None => {
                            return Err(Abort::ThrowString(format!(
                                "Error: 变量已回收(from Eval for Code::Bracket)"
                            )))
                        }
                    }
                    .eval(vm, Rc::clone(&ctx))?;

                    let hc = {
                        let mut tobe: Vec<Var> = Vec::with_capacity(args.len() - 1);
                        for code in args.iter().skip(1) {
                            tobe.push(code.eval(vm, Rc::clone(&ctx))?);
                        }
                        tobe
                    };
                    var.unbox()?.as_func((vm, &hc, Rc::clone(&ctx)))?
                }
            }
            Self::Middle(ref args) => {
                #[cfg(debug_assertions)]
                {
                    println!("尝试执行函数")
                }

                let var = match args.get(0) {
                    Some(fun) => fun,
                    None => {
                        return Err(Abort::ThrowString(format!(
                            "Error: 变量已回收(from Eval for Code::Bracket)"
                        )))
                    }
                }
                .eval(vm, Rc::clone(&ctx))?;
                
                let hc = {
                    let mut tobe: Vec<Var> = Vec::with_capacity(args.len() - 1);
                    for code in args.iter().skip(1) {
                        tobe.push(code.eval(vm, Rc::clone(&ctx))?);
                    }
                    tobe
                };
                var.unbox()?.as_func((vm, &hc, Rc::clone(&ctx)))?
            }
            Self::The(ref c) => c.clone(),
        }
        .into()
    }
}
