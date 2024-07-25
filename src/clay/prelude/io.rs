use crate::clay::{
    var::{func::Args, ToCross},
    vm::{
        signal::{Abort, Signal},
        Eval, Token,
    },
};
use std::io::{BufRead, Write};
use std::rc::Rc;

pub fn puts(args: Args) -> Signal {
    let (vm, args, ctx, ctrl) = args;
    match args.get(0) {
        Some(msg_t) => {
            match msg_t {
                Token::Str(msg) => {
                    println!("{}", msg)
                }
                _ => {
                    let msg_c = msg_t.eval(vm, Rc::clone(&ctx), ctrl)?;
                    match msg_c.uncross()?.cast::<String>() {
                        Some(msg) => println!("{}", msg),
                        None => {
                            return Err(Abort::ThrowString("puts: expected a string".to_string()))
                        }
                    }
                }
            }
            Ok(ctx.get("undef")?)
        }
        None => {
            println!();
            Ok(ctx.get("undef")?)
        }
    }
}

pub fn input(args: Args) -> Signal {
    let (vm, args, ctx, ctrl) = args;
    match args.get(0) {
        Some(msg_t) => match msg_t {
            Token::Str(msg) => {
                Ok(inner_input(msg.to_owned()).to_cross(vm))
            }
            _ => {
                let msg_c = msg_t.eval(vm, Rc::clone(&ctx), ctrl)?;
                match msg_c.uncross()?.cast::<String>() {
                    Some(msg) =>Ok(inner_input(msg.to_owned()).to_cross(vm)),
                    None => return Err(Abort::ThrowString(
                        format!("Error:puts希望第一个参数是字符串,但传入了 {:?}",msg_c)
                    )),
                }
            }
        },
        None => Ok(inner_input("".to_owned()).to_cross(vm)),
    }
}

fn inner_input(msg: String) -> String {
    print!("{} ", msg);

    match std::io::stdout().flush(){
        Ok(_)=>(),
        Err(e)=>panic!("Error: {:?}",e),
    };
    // 读取标准输入
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    // 读取一行输入
    handle.read_line(&mut buffer).expect("Failed to read line");

    // 去除末尾的换行符
    buffer.trim().to_string()
}
