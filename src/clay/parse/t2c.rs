use crate::clay::{
    var::{ToVar, Var},
    vm::{
        runtime::Vm,
        signal::{Abort, ErrSignal},
        Code, Token,
    },
};

pub fn t2c(text: &Vec<Token>, vm: &Vm) -> ErrSignal<Vec<Code>> {
    let vm = *vm;
    let mut result = vec![];

    let text = t2ml(vm, text)?;

    for token in text {
        result.push(tran(vm, &token)?);
    }
    Ok(result)
}

fn tran(vm: Vm, token: &ML) -> ErrSignal<Code> {
    use ML::*;
    let hc = match token {
        Id(ref s) => Code::Id(s.clone()),

        Number(ref f) => Code::The(f.to_var(vm)),

        // Token::Int(ref i) => Ok(Code::The(i.to_owned().to_var(vm))),
        // Token::Float(ref f) => Ok(Code::The(f.to_var(vm))),
        Str(ref s) => Code::The(s.to_owned().to_var(vm)),
        Template(ref s) => Code::Template(s.clone()),

        // Token::Bracket(ref b) => Code::Bracket(b.iter().map(|t| t.format()).collect()),
        // Token::Block(ref b) => Code::Block(b.iter().map(|t| t.format()).collect()),
        // Token::Middle(ref b) => Code::Middle(b.iter().map(|t| t.format()).collect()),
        Large(ref b) => Code::Block({
            let mut hc = Vec::with_capacity(b.len());
            for token in b {
                hc.push(tran(vm, token)?);
            }
            hc
        }),

        Middle(b) => {
            let mut hc = Vec::with_capacity(b.len());
            let mut result = vec![];

            for token in b.into_iter() {
                match token {
                    Comma => {
                        if hc.len() == 0 {
                            continue;
                        } else {
                            result.push(tran(vm, &Line(hc))?);
                            hc = vec![];
                        }
                    }
                    _ => hc.push(token.clone()),
                }
            }

            Code::Array(result)
        }

        Bracket(ref expr) => important(expr)?,

        Line(ref expr) => important(expr)?,

        The(ref c) => Code::The(c.clone()),

        _ => todo!("未实现的t2c"),
    };

    Ok(hc)
}

#[derive(Debug, Clone)]
enum ML {
    Id(String),

    // Int(BigInt),
    // Float(f64),
    Number(f64),

    Str(String),
    Template(String),

    Bracket(Vec<ML>),
    Large(Vec<ML>),
    Middle(Vec<ML>),

    Line(Vec<ML>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),

    Keys(MLKW),

    //"+" | "-" | "*" | "/" | "%" | "==" | "!=" | ">=" | "<="
    //| ">" | "<" | "&&" | "||" | "!" | "neg"
    Op(MLOp),
}

#[derive(Debug, Clone)]
enum MLOp {
    In,
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Eq,
    Ne,
    Ge,
    Le,
    Gt,
    Lt,

    And,
    Or,
    Not,

    None,
}

#[derive(Debug, Clone)]
enum MLKW {
    If,
    Elif,
    Else,

    Lambda,

    Var,
    Func,

    Match,

    Comma,
}

fn t2ml(vm: Vm, text: &Vec<Token>) -> ErrSignal<Vec<ML>> {
    fn tran(vm: Vm, token: &Token) -> ErrSignal<ML> {
        use ML::*;

        let ml = match token {
            Token::Number(ref f) => ML::Number(f.to_owned()),

            Token::The(ref c) => ML::The(c.clone()),

            Token::Id(ref s) => match s.as_str() {
                "\\" | "if" | "elif" | "else" | "var" | "func" | "match" | "," | ";" => 
                    match s.as_str() {
                        "\\" => ML::Keys(MLKW::Lambda),
                        "if" => ML::Keys(MLKW::If),
                        "elif" => ML::Keys(MLKW::Elif),
                        "else" => ML::Keys(MLKW::Else),
                        "var" => ML::Keys(MLKW::Var),
                        "func" => ML::Keys(MLKW::Func),
                        "match" => ML::Keys(MLKW::Match),
                        "," => ML::Keys(MLKW::Comma),
                        ";" => ML::Keys(MLKW::Comma),
                        _ => unreachable!(),
                    },
                "in" | "+" | "-" | "*" | "/" | "%" | "==" | "!=" | ">=" | "<=" | ">" | "<"
                | "&&" | "||" | "!" => {
                    use MLOp::*;
                    Op(match s.as_str() {
                        "in" => In,
                        "+" => Add,
                        "-" => Sub,
                        "*" => Mul,
                        "/" => Div,
                        "%" => Mod,
                        "==" => Eq,
                        "!=" => Ne,
                        ">=" => Ge,
                        "<=" => Le,
                        ">" => Gt,
                        "<" => Lt,
                        "&&" => And,
                        "||" => Or,
                        "!" => Not,
                        _ => unreachable!(),
                    })
                }
                _ => Id(s.clone()),
            },

            Token::Str(ref s) => ML::Str(s.clone()),

            Token::Template(ref s) => ML::Template(s.clone()),

            Token::Line(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm, token)?);
                }
                ML::Line(ml)
            }

            Token::Bracket(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm, token)?);
                }
                ML::Bracket(ml)
            }

            Token::Large(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm, token)?);
                }
                ML::Large(ml)
            }
            Token::Middle(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm, token)?);
                }
                ML::Middle(ml)
            }
        };

        Ok(ml)
    }
    let mut result = vec![];

    for token in text {
        let hc = tran(vm, token)?;

        result.push(hc);
    }

    Ok(result)
}

fn important(expr: &Vec<ML>) -> ErrSignal<Code> {
    use ML::*;
    //todo!("重头戏")
    let mut iter = expr.iter();
    let mut stage1 = vec![];

    while let Some(token) = iter.next() {
        fn closure<'a>(
            iter: &mut impl Iterator<Item = &'a ML>,
            mut hc: Vec<ML>,
            stage1: &mut Vec<(Vec<ML2>, MLOp)>,
        ) -> ErrSignal<()> {

            fn up(mls: Vec<ML>) -> ErrSignal<Vec<ML2>> {
                fn up_a(ml:ML,iter:&mut impl Iterator<Item = ML>)->ErrSignal<ML2>{
                    let hc = match ml{
                        Id(s)=> ML2::Id(s),
                        Number(f)=> ML2::Number(f),
                        Str(s)=> ML2::Str(s),
                        Template(s)=> ML2::Template(s),

                        The(v)=> ML2::The(v),

                        Op(_)=>unreachable!(),

                        Keys(k)=>{
                            match k{
                                MLKW::If=>{
                                    let mut last = false;
                                    let hc = loop{
                                        match iter.next(){
                                            None=> return Err(
                                                Abort::ThrowString("expect expression after if".to_owned())
                                            ),
                                            Some(mm)=>match mm{
                                                
                                            }
                                        }
                                    };
                                    ML2::If(hc)
                                }
                                _=>todo!()
                            }
                        }

                        Bracket(b)=>{
                            let mut result = Vec::with_capacity(b.len());
                            for ml in b{
                                result.push(up_a(ml,iter)?)
                            }
                            ML2::Bracket(result)
                        }

                        Large(b)=>{
                            let mut result = Vec::with_capacity(b.len());
                            for ml in b{
                                result.push(up_a(ml,iter)?)
                            }
                            ML2::Large(result)
                        }

                        Line(b)=>{
                            let mut result = Vec::with_capacity(b.len());
                            for ml in b{
                                result.push(up_a(ml,iter)?)
                            }
                            ML2::Line(result)
                        }

                        Middle(b)=>{
                            let mut result = Vec::with_capacity(b.len());
                            for ml in b{
                                result.push(up_a(ml,iter)?)
                            }
                            ML2::Middle(result)
                        }
                    };
                    Ok(hc)
                }
                let mut result = Vec::with_capacity(mls.len());
                let mut iter = mls.into_iter();

                loop{
                    match iter.next(){
                        Some(ml)=>{
                            let hc = up_a(ml,&mut iter)?;
                            result.push(hc);
                        }
                        None => break,
                    }
                }
                result.shrink_to_fit();
                Ok(result)
            }

            loop {
                match iter.next() {
                    Some(t) => match t {
                        Op(op) => {
                            stage1.push((up(hc)?, op.clone()));
                            break;
                        }
                        _ => hc.push(t.clone()),
                    },
                    None => {
                        stage1.push((up(hc)?, MLOp::None));
                        break;
                    }
                }
            }
            Ok(())
        }
        match token {
            Op(MLOp::Sub) => closure(&mut iter, vec![Id("neg".to_owned())], &mut stage1)?,
            Op(_) => {
                return Err(Abort::ThrowString(format!(
                    "unexpect operator: {:?}",
                    token
                )))
            }
            _ => closure(&mut iter, vec![], &mut stage1)?,
        }
    }

    let _stage1_iter = stage1.into_iter().map(|(mls, op)| (mls, op, 0usize));

    todo!("重头戏: 对运算符进行提升");

    //()
}

enum ML2{
    If(Vec<Vec<(ML2, ML2)>>),

    Lambda(Vec<String>, Box<ML2>),

    Var(String, Box<ML2>),

    Func(Option<String>, Vec<String>, Box<ML2>),

    Match(Box<ML2>, Vec<(ML2, ML2)>),

    Id(String),

    // Int(BigInt),
    // Float(f64),
    Number(f64),

    Str(String),
    Template(String),

    Bracket(Vec<ML2>),
    Large(Vec<ML2>),
    Middle(Vec<ML2>),

    Line(Vec<ML2>),

    //Option(String),
    //Lambda(Vec<String>,Box<Code>),
    The(Var),
}
