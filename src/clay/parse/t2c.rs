use crate::clay::{
    var::{ToVar, Var},
    vm::{runtime::Vm, signal::{Abort, ErrSignal}, Code, Token},
};

pub fn t2c(text: &Vec<Token>, vm: &Vm) -> ErrSignal<Vec<Code>> {
    let vm = *vm;
    let mut result = vec![];

    let text = t2ml(vm,text)?;
    
    for token in text {
        result.push(tran(vm,&token)?);
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

        Middle(b) =>{
            let mut hc = Vec::with_capacity((b.len() - 1) / 2);
            let mut is_dh = false;
            for token in b {
                if is_dh{
                    match token{
                        Id(str)=>{
                            if str == ","{
                                is_dh = false;
                                continue;
                            }else {
                                return Err(Abort::ThrowString(
                                    format!("expect ,")
                                ))
                            }
                        }
                        _ => {
                            is_dh = false;
                            hc.push(tran(vm, token)?);
                        }
                    }
                }
            }
            Code::Array(hc)
        },

        Bracket(ref expr) => {
            important(expr)
        }

        Line(ref expr) => {
            //todo!("重头戏")

            let mut bracket = vec![];
            let mut stage1 = vec![];

            for token in expr.iter() {
                if let Id(ref s) = token {
                    match s.as_str() {
                        "+" | "-" | "*" | "/" | "++" | "--" | "%" | "==" | "!=" | ">=" | "<="
                        | ">" | "<" | "&&" | "||" | "!" | "neg" | "@" | ":" | "~" => {
                            stage1.push(bracket);
                            stage1.push(vec![token]);

                            bracket = vec![];
                        }
                        _ => bracket.push(token),
                    }
                } else {
                    bracket.push(token)
                }
            }

            drop(bracket);

            let _stage1_iter = stage1.into_iter().map(|tv| (tv, 0usize));

            todo!("重头戏: 对运算符进行提升");

            //()
        }

        The(ref c) => Code::The(c.clone()),

        _=>todo!("未实现的t2c")
    };

    Ok(hc)
}

enum ML{
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

    If,Then,Else,

    Lambda,

    Set,

    Do,End,

    Match,

    In,
}

fn t2ml(vm:Vm,text: &Vec<Token>) -> ErrSignal<Vec<ML>> {
    fn tran(vm:Vm,token: &Token) -> ErrSignal<ML> {
        use ML::*;

        let ml = match token {
            Token::Number(ref f) => ML::Number(f.to_owned()),

            Token::The(ref c) => ML::The(c.clone()),

            Token::Id(ref s) => {
                match s.as_str(){
                    "\\"=>Lambda,
                    "if"=>If,
                    "then"=>Then,
                    "else"=>Else,
                    "set"=>Set,
                    "in"=>In,
                    "do"=>Do,
                    "end"=>End,
                    "match"=>Match,
                    _=>Id(s.clone())
                }
            },
            
            Token::Str(ref s) => ML::Str(s.clone()),
            Token::Template(ref s) => ML::Template(s.clone()),
            Token::Line(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm,token)?);
                }
                ML::Line(ml)
            }

            Token::Bracket(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm,token)?);
                }
                ML::Bracket(ml)
            }

            Token::Large(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm,token)?);
                }
                ML::Large(ml)
            }
            Token::Middle(ref b) => {
                let mut ml = vec![];
                for token in b {
                    ml.push(tran(vm,token)?);
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

fn important(expr: &Vec<ML>) -> Code {
    use ML::*;
    //todo!("重头戏")

    let mut bracket = vec![];
    let mut stage1 = vec![];

    for token in expr.iter() {
        if let Id(ref s) = token {
            match s.as_str() {
                "+" | "-" | "*" | "/" | "++" | "--" | "%" | "==" | "!=" | ">=" | "<="
                | ">" | "<" | "&&" | "||" | "!" | "neg" | "@" | ":" | "~" => {
                    stage1.push(bracket);
                    stage1.push(vec![token]);

                    bracket = vec![];
                }
                _ => bracket.push(token),
            }
        } else {
            bracket.push(token)
        }
    }

    drop(bracket);

    let _stage1_iter = stage1.into_iter().map(|tv| (tv, 0usize));

    todo!("重头戏: 对运算符进行提升");

    //()
}