use crate::clay::vm::signal::{Abort, ErrSignal};

use super::args::Args;
pub fn escape(s: &str) -> Result<String, String> {
    let mut iter = s.chars();
    let mut result = String::with_capacity(s.len());
    loop {
        match iter.next() {
            Some(c) => {
                if c == '\\' {
                    match iter.next() {
                        None => return Err("unterminated escape sequence".to_owned()),
                        Some(c) => match c {
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            '0' => result.push('\0'),
                            '"' => result.push('"'),
                            '\'' => result.push('\''),
                            '\\' => result.push('\\'),
                            _ => return Err(format!("unknown escape sequence: \\{}", c)),
                        },
                    }
                } else {
                    result.push(c);
                }
            }
            None => break,
        }
    }
    result.shrink_to_fit();
    Ok(result)
}
pub fn template(s: &str, all: Args) -> ErrSignal<String> {
    let mut iter = s.chars();
    let mut result = String::with_capacity(s.len());

    loop {
        match iter.next() {
            Some(c) => match c {
                '{' => {
                    let mut to_eval = String::new();
                    loop {
                        match iter.next() {
                            Some(c) => match c {
                                '}' => break,
                                _ => to_eval.push(c),
                            },
                            None => {
                                return Err(Abort::ThrowString("unterminated template".to_owned()))
                            }
                        }
                    }
                    to_eval.shrink_to_fit();
                    let hc = all.ctx().unbox()?.get(*all.vm(), &to_eval)?;

                    let b = hc.unbox()?
                        .get(*all.vm(), "toStr")?
                        .unbox()?
                        .call(Args::new(*all.vm(),&[],all.ctx().clone()))?
                        .unbox()?;

                    let r: &String = b.cast()?;
                    result.push_str(r);
                }
                _ => result.push(c),
            },
            None => break,
        }
    }

    result.shrink_to_fit();
    Ok(result)
}
