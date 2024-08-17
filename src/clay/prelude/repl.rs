use crate::clay::prelude::objects::func::{Func, Script};
use crate::clay::var::ToVar;
use crate::clay::vm::env::void_ctx;
use crate::clay::vm::signal::Signal;
use crate::clay::vm::Token;
use crate::clay::{parse, vm};
use std::io::Write;
use std::process::exit;

use super::objects::args::Args;

pub fn repl() -> Signal {
    println!("clay,启动\n输入.exit退出");
    let vm = vm::runtime::Vm::new()?;

    inner_repl(Args::new(vm, &[] as &[_], void_ctx(vm)))
}

pub fn inner_repl(all: Args) -> Signal {
    let _vm = *all.vm();

    {
        let mut _begin = 0;
        let mut end = 0;

        loop {
            print!("\nclay> ");

            //读取键盘输入
            match std::io::stdout().flush() {
                Ok(_) => (),
                Err(e) => panic!("Error: {:?}", e),
            };

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == ".exit" {
                break;
            } else if input.trim().ends_with("{") {
                loop {
                    print!("....~ ");

                    //读取键盘输入
                    match std::io::stdout().flush() {
                        Ok(_) => (),
                        Err(e) => panic!("Error: {:?}", e),
                    };

                    let mut in_input = String::new();
                    std::io::stdin().read_line(&mut in_input).unwrap();

                    input += &in_input;

                    if in_input.trim().ends_with("}") {
                        println!();
                        break;
                    }
                }
            }

            //记录有几行输入

            let line = input.split("\n").collect::<Vec<_>>().len();

            {
                _begin = end;
                end += line;
            }

            let hc = match parse::Parser::new(&input).parse() {
                Ok(hc) => hc,
                Err(e) => {
                    eprintln!("parse error: {}", e);
                    continue;
                }
            };

            #[cfg(debug_assertions)]
            {
                println!("\n{:#?}\n", hc);
            }

            let func = {
                let foo;
                let codes:&[Token] = match hc {
                    Token::Large(codes) =>{
                        foo = codes;
                        &foo
                    },
                    _ => &[hc],
                };
                let script = Script::new(
                    &Some(if _begin == end {
                        format!("line {}", _begin)
                    } else {
                        format!("line {} ~ {}", _begin, end)
                    }),
                    &[],
                    _vm.get_context(),
                    codes,
                );

                Func::Script(script).to_var(_vm)
            };

            match _vm.run_code(func) {
                Ok(v) => {
                    println!(
                        "{}\n",
                        v.unbox()?
                            .get(_vm, "toStr")?
                            .unbox()?
                            .call(Args::new(_vm, &[], _vm.get_context().clone()))?
                            .unbox()?
                            .cast::<String>()?
                    );
                }
                Err(e) => eprintln!("{}", e),
            };
        }

        exit(0);
    }
}
