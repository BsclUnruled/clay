use crate::clay::vm::{env::Env, signal::{Abort, ErrSignal}};

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
pub fn template(s: &str,env:&Env) -> ErrSignal<String> {
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
                    let hc = env.ctx().get(env, &to_eval).sync()?;

                    let b = hc.get(env, "toStr")
                        .call(env,&[])
                        .sync()?;

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
