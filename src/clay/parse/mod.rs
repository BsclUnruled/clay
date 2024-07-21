use super::{var::object::Object, vm::Code};
use std::cell::Cell;
use crate::clay::var::lambda::lambda;

pub struct CodeIter<'a> {
    code: &'a str,
    index: Cell<usize>,
}

type Return = Result<Code, String> ;

impl<'a> CodeIter<'a> {
    pub fn new(code: &'a str) -> Self {
        Self { code, index: 0.into() }
    }

    fn parse_until(&self,end:Option<char>) -> Return{
        let mut token = Vec::<char>::new();
        let mut result:Vec<Code> = vec![];
        loop{
            let next = self.next();
            if end == next{
                if token.len() > 0{
                    result.push(Code::Sym(token.iter().collect::<String>()));
                }
                if result.len() > 0{
                    return Ok(Code::Bracket(result));
                }else{
                    return Ok(Code::Bracket(vec![Code::Sym("".to_owned())]));
                }
            }else{
                match next{
                    Some(c) => match c {
                        '('=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_until(Some(')'))?)
                        },
                        '"'=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_escape()?)
                        },
                        '\''=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_str()?)
                        },
                        '`'=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_template()?)
                        },
                        '\t'|'\r'|' '|'\n' => {
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                        },
                        '\\'=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_lambda()?)
                        },
                        '['=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_until(Some(']'))?)
                        },
                        '{'=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_block()?)
                        },
                        '#'=>{
                            if token.len() > 0{
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![]; 
                            }
                            match self.ignore(){
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            }
                        },
                        '}'|')'|']'=>{
                            return Err("Unexpected closing bracket".to_owned());
                        },
                        _=>token.push(c),
                    },
                    None => return Err("Unexpected end of code".to_owned()),
                }
            }
        }
    }

    fn ignore(&self)->Result<(),String>{
        let result = vec![Code::The(lambda())];
        loop{
            match self.next(){
                Some(c)=>match c{
                    //todo
                }
                None => return Err("Unexpected end of code".to_owned()),
            }
        }
        Ok(())
    }

    fn parse_lambda(&self) -> Return{
        loop{
            match self.next(){
                Some(c) => match c {
                    _ => todo!(),
                },
                None => return Err("Unexpected end of code".to_owned()),
            }
        }
    }

    // pub fn parse(&self) -> Return{
    //     let mut token = Vec::<char>::new();
    //     let mut line = Vec::<Code>::new();
    //     let mut result:Vec<Code> = vec![];
    //     loop{
    //         match self.next() {
    //             Some(c) => match c {
    //                 '{'=>line.push(self.parse_block()?),
    //                 '('=>line.push(self.parse_bracket()?),
    //                 '"'=>line.push(self.parse_escape()?),
    //                 '\''=>line.push(self.parse_str()?),
    //                 '`'=>line.push(self.parse_template()?),
    //                 ' '|'\t'|'\r'=>{
    //                     if token.len() > 0{
    //                         line.push(
    //                             Code::Sym(token.iter().collect::<String>())
    //                         );
    //                         token = vec![];
    //                     }
    //                 },
    //                 '\n'=>{
    //                     if line.len() > 0{
    //                         result.push(Code::Bracket(line));
    //                         line = vec![];
    //                     }
    //                 },
    //                 '#'=>loop{
    //                     match self.next(){
    //                         Some('\n')=> break,
    //                         Some(_) => continue,
    //                         None => return Err("Unexpected end of code".to_owned()),
    //                     }
    //                 },
    //                 ']'|')'|'}'=>return Err("Unexpected closing bracket".to_owned()),
    //                 _=>token.push(c),
    //             },
    //             None => return Ok({
    //                 if token.len() > 0{
    //                     line.push(Code::Sym(token.iter().collect::<String>()))
    //                 }
    //                 if line.len() > 0{
    //                     result.push(Code::Bracket(line));
    //                 }
    //                 Code::Bracket(result)
    //             }),
    //         }
    //     }
    // }

    // fn parse_block(&self) -> Return{
    //     let mut token = Vec::<char>::new();
    //     let mut line = Vec::<Code>::new();
    //     let mut result:Vec<Code> = vec![];
    //     loop{
    //         match self.next(){
    //             Some(c) => match c {
    //                 '{'=>line.push(self.parse_block()?),
    //                 '('=>line.push(self.parse_bracket()?),
    //                 '"'=>line.push(self.parse_escape()?),
    //                 '\''=>line.push(self.parse_str()?),
    //                 '`'=>line.push(self.parse_template()?),
    //                 ' '|'\t'|'\r'=>{
    //                     if token.len() > 0{
    //                         line.push(
    //                             Code::Sym(token.iter().collect::<String>())
    //                         );
    //                         token = vec![];
    //                     }
    //                 },
    //                 '\n'=>{
    //                     if line.len() > 0{
    //                         result.push(Code::Bracket(line));
    //                         line = vec![];
    //                     }
    //                 },
    //                 '#'=>loop{
    //                     match self.next(){
    //                         Some('\n')=> break,
    //                         Some(_) => continue,
    //                         None => return Err("Unexpected end of code".to_owned()),
    //                     }
    //                 },
    //                 '}'=>return Ok({
    //                     if token.len() > 0{
    //                         line.push(Code::Sym(token.iter().collect::<String>()))
    //                     }
    //                     if line.len() > 0{
    //                         result.push(Code::Bracket(line));
    //                     }
    //                     Code::Bracket(result)
    //                 }),
    //                 ')'|']'=>return Err("Unexpected closing bracket".to_owned()),
    //                 _=>token.push(c),
    //             },
    //             None => return Err("Unexpected end of code".to_owned()),
    //         }
    //     }
    // }

    fn parse_block(&self) -> Return{
        todo!()
    }

    fn parse_bracket(&self) -> Return{
        self.parse_until(Some(')'))
    }

    fn parse_escape(&self) -> Return{
        loop{
            match self.next(){
                Some(c) => match c {
                    _=>todo!(),
                },
                None => return Err("Unexpected end of code".to_owned()),
            }
        }
    }

    fn parse_str(&self) -> Return{
        loop{
            match self.next(){
                Some(c) => match c {
                    _=>todo!(),
                },
                None => return Err("Unexpected end of code".to_owned()),
            }
        }
    }

    fn parse_template(&self) -> Return{
        loop{
            match self.next(){
                Some(c) => match c {
                    _=>todo!(),
                },
                None => return Err("Unexpected end of code".to_owned()),
            }
        }
    }


    fn done(&self) -> bool {
        self.index.get() >= self.code.len()
    }

    fn next(&self) -> Option<char> {
        match self.code.chars().nth(self.index.get()) {
            Some(c) => {
                self.index.set(self.index.get() + 1);
                Some(c)
            }
            None => None,
        }
    }
}

// impl CodeIter{
//     pub fn next(self)->Option<char>{}
// }

// pub fn parse_top(text: &str)->Result<Code, ParseError>{
//     let iter = CodeIter{code:text, index:0};

//     let mut codes = Vec::<char>::new();
//     for c in iter{

//     };
//     Ok(Code::The(undef()))
// }

// pub fn parse_block<T>
//     (text:&T)
//     -> Result<Code, ParseError>
//     where T:Iterator<Item=char>{
//     let iter = text.peekable();
//     let mut hc = vec![];
//     let mut sym_part = vec![];
//     for c in iter{
//         match c{
//             '}'=>return Ok(Code::Block(hc)),
//             '('=>match parse_bracket(text){
//                 Ok(code)=>{
//                     hc.push(code);
//                 },
//                 Err(e)=>return Err(e)
//             },
//             _=>sym_part.push(c)
//         }
//     }
//     Ok(Code::The(undef()))
// }

// pub fn parse_bracket<T>
//     (text:&T)
//     -> Result<Code, ParseError>
//     where T:Iterator<Item=char>{
//     todo!()
// }
