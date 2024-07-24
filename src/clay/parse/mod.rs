use num_bigint::BigInt;

use super::{
    var::string,
    vm::Code,
};
use std::{cell::Cell, collections::LinkedList, str::FromStr};

pub struct Parser<'a> {
    code: &'a str,
    index: Cell<usize>,
}

type Return = Result<Code, String>;

fn closure(result: &mut Vec<Code>, token:&mut Vec<char>) -> Result<(), String> {
    if token.len() > 0 {
        let t = token.iter().collect::<String>();
        *token = vec![];
        
        let yes = {
            //获取str第一个字符
            let hc = t.chars().next().unwrap();
            //判断是否为数字开头
            hc.is_numeric() || hc == '-'
        };

        if yes {
            return match BigInt::from_str(&t) {
                Ok(n) => {
                    result.push(Code::Int(n));
                    Ok(())
                }
                Err(_) => match f64::from_str(&t) {
                    Ok(n) => {
                        result.push(Code::Float(n));
                        Ok(())
                    }
                    Err(_) => Err(format!("Invalid number: {}", t)),
                },
            };
        } else {
            result.push(Code::Token(t));
            Ok(())
        }
    } else {
        Ok(())
    }
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            index: 0.into(),
        }
    }

    pub fn parse(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("start parse");

        let mut result: Vec<Code> = vec![];

        while !self.done() {
            let (code, end) = self.parse_line_unless(None)?;
            match code {
                Some(c) => result.push(c),
                None => (),
            };
            if end {
                #[cfg(debug_assertions)]
                println!("finish parse");

                return Ok(Code::Block(result));
            } else {
                continue;
            }
        }
        return Ok(Code::Block(result));
    }

    //                                                      结果，是否以end结尾
    fn parse_line_unless(&self, end: Option<char>) -> Result<(Option<Code>, bool), String> {
        #[cfg(debug_assertions)]
        println!(
            "parse_line_unless_{:?}",
            match end {
                Some(c) => format!("{}", c),
                None => "None".to_owned(),
            }
        );

        let mut token = Vec::<char>::new();
        let mut result: Vec<Code> = vec![];
        loop {
            let next = self.next();
            if next == end {
                #[cfg(debug_assertions)]
                println!("finish parse_line_unless_{:?} (end with {:?})", end, next);

                return Ok((
                    {
                        closure(&mut result,&mut token)?;
                        if result.len() > 0 {
                            Some(Code::Bracket(result))
                        } else {
                            None
                        }
                    },
                    true,
                ));
            } else {
                match next {
                    Some(c) => match c {
                        '(' => {
                            closure(&mut result,&mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_bracket({:?}): ", c);

                            result.push(self.parse_bracket(')')?)
                        }
                        '"' => {
                            closure(&mut result,&mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_escape: ");

                            result.push(self.parse_escape()?)
                        }
                        '\'' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_str: ");

                            result.push(self.parse_str()?)
                        }
                        '`' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_template:");

                            result.push(self.parse_template()?)
                        }
                        '\t' | '\r' | ' ' => {
                            closure(&mut result, &mut token)?;
                        }
                        '\n' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("finish parse_line_unless_{:?} (end with \\n)", end);

                            return Ok((
                                if result.len() > 0 {
                                    Some(Code::Bracket(result))
                                } else {
                                    None
                                },
                                false,
                            ));
                        }
                        '\\' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_lambda:");

                            result.push(self.parse_lambda()?)
                        }
                        '[' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_bracket({:?}):", c);

                            result.push(self.parse_bracket(']')?)
                        }
                        '{' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_block:");

                            result.push(self.parse_block()?)
                        }
                        '#' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use ignore:");

                            match self.ignore() {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            }
                        }
                        '}' | ')' | ']' => {
                            return Err(format!("Unexpected closing bracket: {:?}", c));
                        }
                        _ => token.push(c),
                    },
                    None => {
                        return Err(format!(
                            "Unexpected end of code(from parse_line_unless_{:?})",
                            match end {
                                Some(c) => format!("{}", c),
                                None => "None".to_owned(),
                            }
                        ))
                    }
                }
            }
        }
    }

    fn ignore(&self) -> Result<(), String> {
        #[cfg(debug_assertions)]
        println!("ignore");

        loop {
            match self.next() {
                Some(c) => match c {
                    '\n' => {
                        #[cfg(debug_assertions)]
                        println!("finish ignore");

                        return Ok(());
                    }
                    _ => continue,
                },
                None => return Err("Unexpected end of code(from ignore)".to_owned()),
            }
        }
    }

    fn parse_lambda(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_lambda");

        Ok(Code::Bracket(vec![
            Code::Token("\\".to_owned()),
            {
                loop {
                    match self.next() {
                        Some('(') => break,
                        Some(c) => match c {
                            '\n' | '\t' | ' ' | '\r' => continue,
                            _ => {
                                return Err(format!(
                                    "Unexpected character {:?}(from parse_lambda->parse_bracket)",
                                    c
                                ))
                            }
                        },
                        None => {
                            return Err("Unexpected end of code(from parse_lambda->parse_bracket)"
                                .to_owned())
                        }
                    }
                }
                self.parse_bracket(')')
            }?,
            {
                loop {
                    match self.next() {
                        Some('{') => break,
                        Some(c) => match c {
                            '\n' | '\t' | ' ' | '\r' => continue,
                            _ => {
                                return Err(format!(
                                    "Unexpected character {:?}(from parse_lambda->parse_block)",
                                    c
                                ))
                            }
                        },
                        None => {
                            return Err(
                                "Unexpected end of code(from parse_lambda->parse_block)".to_owned()
                            )
                        }
                    }
                }
                self.parse_block()
            }?,
        ]))
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

    fn parse_block(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_block");

        let mut result: Vec<Code> = vec![];

        while !self.done() {
            let (code, end) = self.parse_line_unless(Some('}'))?;
            match code {
                Some(c) => result.push(c),
                None => (),
            };
            if end {
                #[cfg(debug_assertions)]
                println!("finish parse_block");

                return Ok(Code::Block(result));
            } else {
                continue;
            }
        }
        Err("Unexpected end of code(from parse_block)".to_owned())
    }

    fn parse_bracket(&self, end: char) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_bracket(until {:?})", end);

        if end != ')' && end != ']' {
            return Err(format!("Unexpected closing bracket(use {:?})", end));
        }

        let mut token = Vec::<char>::new();
        let mut result: Vec<Code> = vec![];
        loop {
            let next = self.next();
            match next {
                Some(c) if c == end => {
                    #[cfg(debug_assertions)]
                    println!("finish parse_bracket(until {:?})", end);
                    if end == ')' {
                        return Ok(Code::Bracket({
                            closure(&mut result, &mut token)?;
                            result
                        }));
                    } else if c == ']' {
                        return Ok(Code::Middle({
                            closure(&mut result, &mut token)?;
                            result
                        }));
                    }
                }
                Some(c) => match c {
                    '(' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_bracket({:?}): ", c);

                        result.push(self.parse_bracket(')')?)
                    }
                    '"' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_escape: ");

                        result.push(self.parse_escape()?)
                    }
                    '\'' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_str: ");

                        result.push(self.parse_str()?)
                    }
                    '`' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_template: ");

                        result.push(self.parse_template()?)
                    }
                    '\t' | '\r' | ' ' | '\n' => {
                        closure(&mut result, &mut token)?;
                    }
                    '\\' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_lambda: ");

                        result.push(self.parse_lambda()?)
                    }
                    '[' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_bracket({:?}): ", c);

                        result.push(self.parse_bracket(']')?)
                    }
                    '{' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use parse_block: ");

                        result.push(self.parse_block()?)
                    }
                    '#' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("use ignore: ");

                        match self.ignore() {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                    }
                    '}' | ')' | ']' => {
                        return Err(format!(
                            "Unexpected closing bracket(expected {:?},found {:?})",
                            end, c
                        ));
                    }
                    _ => token.push(c),
                },
                None => {
                    return Err(format!(
                        "Unexpected end of code(expected {:?} ,found None)",
                        end
                    ))
                }
            }
        }
    }

    fn parse_escape(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_escape");

        let mut string = LinkedList::new();
        let mut escape = false;
        let result = loop {
            match self.next() {
                Some(c) => match c {
                    '\\' => {
                        if !escape {
                            escape = true;
                        } else {
                            string.push_back('\\'); //还原原str中的\\
                            string.push_back('\\');
                            escape = false;
                        }
                    }
                    '"' => {
                        if !escape {
                            #[cfg(debug_assertions)]
                            println!("finish");

                            break string.iter().collect::<String>();
                        } else {
                            string.push_back('"');
                            escape = false;
                        }
                    }
                    _ => string.push_back(c),
                },
                None => return Err("Unexpected end of code(from parse_escape)".to_owned()),
            }
        };
        Ok(Code::Str(string::escape(&result)))
    }

    fn parse_str(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_str");

        let mut string = LinkedList::new();
        loop {
            match self.next() {
                Some('\'') => {
                    #[cfg(debug_assertions)]
                    println!("finish");

                    return Ok(Code::Str(string.iter().collect::<String>()));
                }
                Some(c) => string.push_back(c),
                None => return Err("Unexpected end of code(from parse_str)".to_owned()),
            }
        }
    }

    fn parse_template(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_template");

        let mut string = LinkedList::new();
        let mut escape = false;
        let result = loop {
            match self.next() {
                Some(c) => match c {
                    '\\' => {
                        if !escape {
                            escape = true;
                        } else {
                            string.push_back('\\'); //还原原str中的\\
                            string.push_back('\\');
                            escape = false;
                        }
                    }
                    '`' => {
                        if !escape {
                            #[cfg(debug_assertions)]
                            println!("finish");
                            break string.iter().collect::<String>();
                        } else {
                            string.push_back('"');
                            escape = false;
                        }
                    }
                    _ => string.push_back(c),
                },
                None => return Err("Unexpected end of code(from parse_template)".to_owned()),
            }
        };
        Ok(Code::Template(result))
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
// }panic
