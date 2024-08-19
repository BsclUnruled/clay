use crate::clay::{prelude::objects::{args::Args, string}, var::ToVar};
use super::var::Var;
use signal::{Abort, ErrSignal, Signal};
pub use runtime::Runtime;
pub mod env;
pub mod error;
pub mod gc;
pub mod runtime;
pub mod signal;
pub mod keys;


// #[derive(Debug,Clone)]
// pub enum Token {
//     Id(String),
//     Symbol(String),

//     Comma,

//     // Int(BigInt),
//     // Float(f64),

//     Number(f64),

//     Str(String),
//     Template(String),

//     Bracket(Vec<Token>),
//     Large(Vec<Token>),
//     Middle(Vec<Token>),

//     //Option(String),
//     //Lambda(Vec<String>,Box<Code>),
//     The(Var),
// }

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
    fn eval(&self, all:Args) -> Signal;
}

// impl Eval for Token{
//     fn eval(&self, all:Args) -> Signal {
//         use Token::*;
//         match self {
//             Id(name) => {
//                 all.ctx()
//                    .unbox()?
//                    .get(*all.vm(), &name)
//             }
//             Number(num) => Ok(num.to_var(*all.vm())),
//             Str(s) => Ok(s.to_owned().to_var(*all.vm())),
//             Template(s) => {
//                 let vm = all.vm();
//                 Ok(string::template(s, all.clone())?.to_var(*vm))
//             },
//             Bracket(tokens) => {
//                 match tokens.get(0){
//                     _=>todo!()
//                 }
//             },
//             Large(tokens) => {
//                 let mut re = all.vm().undef()?;

//                 for token in tokens {
//                     re = token.eval(all.clone())?;
//                     all.ctx().unbox()?.set(*all.vm(), "it", &re)?;
//                 }

//                 Ok(re)
//             },
//             Middle(_)=>todo!(),
//             The(var) => Ok(var.clone()),
//             _=>todo!()
//         }
//     }
// }

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

// impl Token{
//     pub fn into_ast(self)->ErrSignal<Ast>{
//         let hc = {
//             use Token::*;
//             match self{
//                 Id(name)=>Ast::Id(name),
//                 Number(num)=>Ast::Number(num),
//                 Str(s)=>Ast::Str(s),
//                 Template(s)=>Ast::Template(s),
//                 Bracket(tokens)=>Ast::Block(tokens.into_iter().map(|t|t.into_ast().unwrap()).collect()),
//                 Large(tokens)=>Ast::Block({
//                     let mut hc = Vec::with_capacity(tokens.len());
//                     let mut it = tokens.into_iter();

//                     for token in it {
//                         let hc = match token{
//                             Id(name)=>match name.as_str(){
//                                 "if" => todo!(),
//                                 "then" => todo!(),
//                                 "else" => todo!(),
//                                 "match" => todo!(),
//                                 "case" => todo!(),
//                                 "do" => todo!(),
//                                 "end" => todo!(),
//                                 _=> Ast::TryApply { func: Box::new(Ast::Id(name)), args: [].into() }
//                             }
//                             S
//                         };
//                     }

//                     hc.into_boxed_slice()
//                 }),
//                 Middle(mut tokens)=>{
//                     let mut hc = Vec::with_capacity(tokens.len());

//                     fn seek(t:&Token)->bool{
//                         match t{
//                             Token::Comma=>true,
//                             _=>false
//                         }
//                     }

//                     while tokens.len() > 0{
//                         let rest = tokens.split_off(
//                             match tokens.iter().position(seek){
//                                 Some(i)=>i,
//                                 None=>tokens.len()
//                             }
//                         );

//                         tokens.remove(0);//移除分隔符

//                         let mut it = rest.into_iter();
//                         if let Some(head) = it.next() {
//                             hc.push(Ast::TryApply {
//                                 func: head.into_ast()?.into(),
//                                  args: {
//                                     it.map(|t|t.into_ast())
//                                         .fold(Ok(Box::new([]) as AstVec), |init, item|match init {
//                                                  Ok(ast) => Ok([ast, Box::new([item?])].concat().into_boxed_slice()),
//                                                  Err(e) => Err(e)
//                                         })?
//                                  }
//                             })
//                         }
//                     }

//                     Ast::Array(hc.into_boxed_slice())
//                 }
//                 The(var)=>Ast::The(var),
//                 Symbol(s)=>Ast::Id(s),
//                 Comma=>Ast::None//TODO: 记录
//             }
//         };

//         Ok(hc)
//     }
// }
