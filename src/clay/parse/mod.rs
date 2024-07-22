use super::{var::string, vm::Code};
use crate::clay::var::lambda::lambda;
use std::{cell::Cell, collections::LinkedList};

pub struct Parser<'a> {
    code: &'a str,
    index: Cell<usize>,
}

type Return = Result<Code, String>;

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

        let mut result = vec![];

        while !self.done() {
            let (code, end) = self.parse_line_unless(None)?;
            result.push(code);
            if end {
                return Ok(Code::Block(result));
            } else {
                continue;
            }
        }
        Err("Unexpected end of code".to_owned())
    }

    //                                                      结果，是否以end结尾
    fn parse_line_unless(&self, end: Option<char>) -> Result<(Code, bool), String> {
        #[cfg(debug_assertions)]
        println!("parse_line_unless_{}", match end {
            Some(c) => format!("{}", c),
            None => "none".to_owned(),
        });

        let mut token = Vec::<char>::new();
        let mut result: Vec<Code> = vec![];
        loop {
            let next = self.next();
            if next == end {
                return Ok((
                    Code::Bracket({
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                        }
                        result
                    }),
                    true,
                ));
            } else {
                match next {
                    Some(c) => match c {
                        '(' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_bracket(')')?)
                        }
                        '"' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_escape()?)
                        }
                        '\'' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_str()?)
                        }
                        '`' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_template()?)
                        }
                        '\t' | '\r' | ' ' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                        }
                        '\n' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                            }

                            return Ok((Code::Bracket(result), false));
                        }
                        '\\' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_lambda()?)
                        }
                        '[' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_bracket(']')?)
                        }
                        '{' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            result.push(self.parse_block()?)
                        }
                        '#' => {
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                                token = vec![];
                            }
                            match self.ignore() {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            }
                        }
                        '}' | ')' | ']' => {
                            return Err(format!("Unexpected closing bracket: {}", c));
                        }
                        _ => token.push(c),
                    },
                    None => return Err(format!(
                        "Unexpected end of code(from parse_line_unless_{})",
                        match end{
                            Some(c)=>format!("{}", c),
                            None=>"none".to_owned(),
                        }
                    )),
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
                    '\n' => return Ok(()),
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
            Code::The(lambda()),
            self.parse_bracket(')')?,
            self.parse_block()?,
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

        let mut result = vec![];

        while !self.done() {
            let (code, end) = self.parse_line_unless(Some('}'))?;
            result.push(code);
            if end {
                return Ok(Code::Block(result));
            } else {
                continue;
            }
        }
        Err("Unexpected end of code(from parse_block)".to_owned())
    }

    fn parse_bracket(&self, end: char) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_bracket({})", end);

        if end != ')' && end != ']' {
            return Err(format!("Unexpected closing bracket(use '{}')",end));
        }

        let mut token = Vec::<char>::new();
        let mut result: Vec<Code> = vec![];
        loop {
            let next = self.next();
            match next {
                Some(c)if c == end => {
                    if end == ')' {
                        return Ok(Code::Bracket({
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                            }
                            result
                        }));
                    }else if c == ']' {
                        return Ok(Code::Middle({
                            if token.len() > 0 {
                                result.push(Code::Sym(token.iter().collect::<String>()));
                            }
                            result
                        }));
                    }
                }
                Some(c) => match c {
                    '(' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_bracket(')')?)
                    }
                    '"' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_escape()?)
                    }
                    '\'' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_str()?)
                    }
                    '`' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_template()?)
                    }
                    '\t' | '\r' | ' '|'\n' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                    }
                    '\\' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_lambda()?)
                    }
                    '[' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_bracket(']')?)
                    }
                    '{' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        result.push(self.parse_block()?)
                    }
                    '#' => {
                        if token.len() > 0 {
                            result.push(Code::Sym(token.iter().collect::<String>()));
                            token = vec![];
                        }
                        match self.ignore() {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                    }
                    '}' | ')' | ']' => {
                        return Err(format!("Unexpected closing bracket(expected {},found {})",end,c));
                    }
                    _ => token.push(c),
                },
                None => return Err(format!("Unexpected end of code(expected '{}' ,found None)",end)),
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
