use super::{var::Number, vm::Token};
use crate::clay::prelude::objects::string;
use std::{cell::Cell, collections::LinkedList, str::FromStr};

// pub mod clay;
// pub use clay as new_parser;

pub struct Parser<'a> {
    code: &'a str,
    index: Cell<usize>,
}

type Return = Result<Token, String>;

fn closure(result: &mut Vec<Token>, token: &mut Vec<char>) -> Result<(), String> {
    if token.len() > 0 {
        let t = token.iter().collect::<String>();
        *token = vec![];

        let yes = {
            //获取str第一个字符
            let hc = t.chars().next().unwrap();
            //判断是否为数字开头
            hc.is_numeric()
        };

        if yes {
            match Number::from_str(&t.replace("_", "")) {
                Ok(n) => {
                    result.push(Token::Number(n));
                    Ok(())
                }
                Err(_) => Err(format!("Invalid number: {}", t)),
            }
        } else {
            result.push(Token::Id(t));
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

    pub fn parse(&self) -> Result<Token, String> {
        #[cfg(debug_assertions)]
        println!("start parse");

        let mut result: Vec<Token> = vec![];

        while !self.done() {
            let (code, end) = self.parse_line_unless(None)?;
            match code {
                Some(c) => result.push(c),
                None => (),
            };
            if end {
                #[cfg(debug_assertions)]
                println!("finish parse");

                return Ok(Token::Large(result));
            } else {
                continue;
            }
        }
        return Ok(Token::Large(result));
    }

    fn peek(&self) -> Option<char> {
        match self.code.chars().nth(self.index.get()) {
            Some(c) => Some(c),
            None => None,
        }
    }

    // // + - * / % ^ ! & | < > =   -> <-
    // // ++ -- ** // += -= *= /= %= == != <= >= && ||
    // // : @ ~
    // fn parse_symbol(&self, sym: char) -> Return {
    //     #[cfg(debug_assertions)]
    //     println!("parse_symbol");

    //     let mut token = vec![sym];

    //     match sym {
    //         ':' | '@' | '~' => {
    //             return Ok(Token::Id(token.iter().collect()));
    //         }
    //         '+' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '+' | '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '-' => {
    //             if let Some(c) = self.peek() {
    //                 match c {
    //                     '=' | '>' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     '-' => {
    //                         #[cfg(debug_assertions)]
    //                         println!("use ignore:");

    //                         match self.ignore() {
    //                             Ok(_) => {}
    //                             Err(e) => return Err(e),
    //                         }
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 }
    //             };
    //             return Ok(Token::Id(token.iter().collect()));
    //         }
    //         '*' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '*' | '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '/' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '/' | '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '%' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '^' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '!' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '&' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '&' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '|' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '|' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '>' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '<' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '=' | '-' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         '=' => {
    //             match self.peek() {
    //                 Some(c) => match c {
    //                     '=' => {
    //                         token.push(c);
    //                         self.next();
    //                         return Ok(Token::Id(token.iter().collect()));
    //                     }
    //                     _ => return Ok(Token::Id(token.iter().collect())),
    //                 },
    //                 None => {
    //                     return Err(format!(
    //                         "Unexpected end of code(from parse_symbol({:?}))",
    //                         sym
    //                     ))
    //                 }
    //             }
    //         }
    //         _ => return Ok(Token::Id(token.iter().collect())),
    //     };
    // }

    //                                                      结果，是否以end结尾
    fn parse_line_unless(&self, end: Option<char>) -> Result<(Option<Token>, bool), String> {
        #[cfg(debug_assertions)]
        println!(
            "parse_line_unless_{:?}",
            match end {
                Some(c) => format!("{}", c),
                None => "None".to_owned(),
            }
        );

        let mut token = Vec::<char>::new();
        let mut result: Vec<Token> = vec![];
        loop {
            let next = self.next();
            if next == end {
                #[cfg(debug_assertions)]
                println!("finish parse_line_unless_{:?} (end with {:?})", end, next);

                return Ok((
                    {
                        closure(&mut result, &mut token)?;
                        if result.len() > 0 {
                            Some(Token::Bracket(result))
                        } else {
                            None
                        }
                    },
                    true,
                ));
            } else {
                match next {
                    Some(c) => match c {
                        // '+' | '-' | '*' | '/' | '%' | '^' | '!' | '&' | '|' | '<' | '>' | '='
                        // | ':' | '@' | '~' | '\\' => {
                        //     closure(&mut result, &mut token)?;

                        //     #[cfg(debug_assertions)]
                        //     println!("use parse_symbol({:?}): ", c);

                        //     result.push(self.parse_symbol(c)?)
                        // }
                        '-' => {
                            if let Some(in_c) = self.peek() {
                                if in_c == '-' {
                                    #[cfg(debug_assertions)]
                                    println!("use ignore:");

                                    match self.ignore() {
                                        Ok(_) => {}
                                        Err(e) => return Err(e),
                                    }
                                } else {
                                    token.push(c);
                                }
                            }
                        }
                        '(' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("use parse_bracket({:?}): ", c);

                            result.push(self.parse_bracket(')')?)
                        }
                        ',' | ';' => {
                            closure(&mut result, &mut token)?;

                            #[cfg(debug_assertions)]
                            println!("collect {:?}", c);

                            token.push(c);

                            closure(&mut result, &mut token)?;
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
                                    Some(Token::Bracket(result))
                                } else {
                                    None
                                },
                                false,
                            ));
                        }
                        // '\\' => {
                        //     closure(&mut result, &mut token)?;

                        //     #[cfg(debug_assertions)]
                        //     println!("use parse_lambda:");

                        //     result.push(self.parse_lambda()?)
                        // }
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

                            result.push(self.parse_block()?);
                        }
                        // '#' => {
                        //     closure(&mut result, &mut token)?;

                        //     #[cfg(debug_assertions)]
                        //     println!("use ignore:");

                        //     match self.ignore() {
                        //         Ok(_) => {}
                        //         Err(e) => return Err(e),
                        //     }
                        // }
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

    // fn parse_lambda(&self) -> Return {
    //     #[cfg(debug_assertions)]
    //     println!("parse_lambda");

    //     Ok(Token::Bracket(vec![
    //         Token::Id("\\".to_owned()),
    //         {
    //             loop {
    //                 match self.next() {
    //                     Some('(') => break,
    //                     Some(c) => match c {
    //                         '\n' | '\t' | ' ' | '\r' => continue,
    //                         _ => {
    //                             return Err(format!(
    //                                 "Unexpected character {:?}(from parse_lambda->parse_bracket)",
    //                                 c
    //                             ))
    //                         }
    //                     },
    //                     None => {
    //                         return Err("Unexpected end of code(from parse_lambda->parse_bracket)"
    //                             .to_owned())
    //                     }
    //                 }
    //             }
    //             self.parse_bracket(')')
    //         }?,
    //         {
    //             loop {
    //                 match self.next() {
    //                     Some('{') => break,
    //                     Some(c) => match c {
    //                         '\n' | '\t' | ' ' | '\r' => continue,
    //                         _ => {
    //                             return Err(format!(
    //                                 "Unexpected character {:?}(from parse_lambda->parse_block)",
    //                                 c
    //                             ))
    //                         }
    //                     },
    //                     None => {
    //                         return Err(
    //                             "Unexpected end of code(from parse_lambda->parse_block)".to_owned()
    //                         )
    //                     }
    //                 }
    //             }
    //             self.parse_block()
    //         }?,
    //     ]))
    // }

    fn parse_block(&self) -> Return {
        #[cfg(debug_assertions)]
        println!("parse_block");

        let mut result: Vec<Token> = vec![];

        while !self.done() {
            let (code, end) = self.parse_line_unless(Some('}'))?;
            match code {
                Some(c) => result.push(c),
                None => (),
            };
            if end {
                #[cfg(debug_assertions)]
                println!("finish parse_block");

                return Ok(Token::Large(result));
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
        let mut result: Vec<Token> = vec![];
        loop {
            let next = self.next();
            match next {
                Some(c) if c == end => {
                    #[cfg(debug_assertions)]
                    println!("finish parse_bracket(until {:?})", end);
                    if end == ')' {
                        return Ok(Token::Bracket({
                            closure(&mut result, &mut token)?;
                            result
                        }));
                    } else if c == ']' {
                        return Ok(Token::Middle({
                            closure(&mut result, &mut token)?;
                            result
                        }));
                    }
                }
                Some(c) => match c {
                    // '+' | '-' | '*' | '/' | '%' | '^' | '!' | '&' | '|' | '<' | '>' | '=' | ':'
                    // | '@' | '~' => {
                    //     closure(&mut result, &mut token)?;

                    //     #[cfg(debug_assertions)]
                    //     println!("use parse_symbol({:?}): ", c);

                    //     result.push(self.parse_symbol(c)?)
                    // }
                    '-' => {
                            if let Some(in_c) = self.peek() {
                                if in_c == '-' {
                                    #[cfg(debug_assertions)]
                                    println!("use ignore:");

                                    match self.ignore() {
                                        Ok(_) => {}
                                        Err(e) => return Err(e),
                                    }
                                } else {
                                    token.push(c);
                                }
                            }
                        }
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
                    // '\\' => {
                    //     closure(&mut result, &mut token)?;

                    //     #[cfg(debug_assertions)]
                    //     println!("use parse_lambda: ");

                    //     result.push(self.parse_lambda()?)
                    // }
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
                    ',' | ';' => {
                        closure(&mut result, &mut token)?;

                        #[cfg(debug_assertions)]
                        println!("collect {:?}", c);

                        token.push(c);

                        closure(&mut result, &mut token)?;
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
                            string.push_back('\\');
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
                    _ => {
                        string.push_back(c);
                        escape = false;
                    }
                },
                None => return Err("Unexpected end of code(from parse_escape)".to_owned()),
            }
        };
        Ok(Token::Str(string::escape(&result)?))
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

                    return Ok(Token::Str(string.iter().collect::<String>()));
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
                            string.push_back('\\');
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
                            string.push_back('`');
                            escape = false;
                        }
                    }
                    _ => {
                        string.push_back(c);
                        escape = false;
                    }
                },
                None => return Err("Unexpected end of code(from parse_template)".to_owned()),
            }
        };
        Ok(Token::Template(result))
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
