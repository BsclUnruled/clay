use super::{prelude::objects::{func::{script, Func}, string}, var::{ToVar, Var, VarBox}};
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


#[derive(Debug, Clone)]
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

                let stage1_iter = stage1.into_iter().map(|tv|(tv,0usize));

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

    Apply(CodeVec),
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

pub trait Eval {
    fn eval(&self, vm: Vm, ctx: Rc<VarBox>) -> Signal;
}

impl Eval for String {
    fn eval(&self, vm: Vm, ctx: Rc<VarBox>) -> Signal {
        use crate::clay::parse::Parser;
        let parser = Parser::new(self);
        match parser.parse() {
            Ok(code) => code.format(vm)?.eval(vm, ctx),
            Err(e) => Err(Abort::ThrowString(e)),
        }
    }
}

impl Eval for [Code] {
    fn eval(&self, vm: Vm, ctx: Rc<VarBox>) -> Signal {
        match self.get(0) {
            None => vm.undef(),
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
    fn eval(&self, vm: Vm, ctx: Rc<VarBox>) -> Signal {
        match self {
            Self::Id(ref s) => ctx.get(vm,s)?,
            Self::The(ref c) => c.clone(),
            Self::Template(ref s) => string::template(s, ctx).to_var(vm),
            Self::Block(ref b) => {
                let new_ctx = env::default(vm,Some(Rc::clone(&ctx)));

                let mut result = vm.undef()?;

                for line in b {
                    result = line.eval(vm, Rc::clone(&new_ctx))?;
                }
                result
            }
            Self::Func { name, args_names, body }=>{
                let script = script::Script::new(
                    &name,
                    args_names,
                    &ctx,
                    &body
                );
                Func::Script(script).to_var(vm)
            }
            Self::Apply(ref args) => {
                Self::apply_expr(vm, args, &ctx)?
            }
            Code::Break(ref expr)=> return  Err(
                Abort::Break(
                    Code::apply_expr(vm,expr,&ctx)?
                )
            ),
            Code::Continue(_)=> return  Err(
                Abort::Continue
            ),
            Code::Return(ref expr)=> return  Err(
                Abort::Return(
                    Code::apply_expr(vm,expr,&ctx)?
                )
            ),
            Code::Throw(ref expr)=> return  Err(
                Abort::Throw(Code::apply_expr(vm, expr, &ctx)?)
            ),
            Code::If(ref cond, ref then, ref else_then) => {
                let cond_rc = Code::apply_expr(vm, cond, &ctx)?.unbox()?;
                let cond_bool = *cond_rc.cast()?;
                if cond_bool {
                    Code::apply_expr(vm, then, &ctx)?
                }else{
                    Code::apply_expr(vm, else_then, &ctx)?
                }
            }
            Code::Try(ref try_block, ref catch_block) => {
                let hc = Code::apply_expr(vm, try_block, &ctx);
                if let Err(Abort::Throw(e)) = hc {
                    Code::apply_expr(vm, &catch_block, &ctx)?
                        .unbox()?
                        .as_func((vm, &[e], Rc::clone(&ctx)))
                } else{
                    hc
                }?
            }
            Code::Op{op,x,y}=>match op{
                _=>todo!("运算符")
            }
        }
        .into()
    }
}

impl Code{
    pub fn apply_expr(vm: Vm, expr: &[Code], ctx:&Rc<VarBox>) -> Signal {
        //至少有两个元素
        let var = expr[0].eval(vm, Rc::clone(&ctx))?;

        let hc = {
            let mut tobe: Vec<Var> = Vec::with_capacity(expr.len() - 1);
            for code in expr.iter().skip(1) {
                tobe.push(code.eval(vm, Rc::clone(&ctx))?);
            }
            tobe
        };
        var.unbox()?.as_func((vm, &hc, Rc::clone(&ctx)))
    }
}
