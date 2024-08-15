use std::io::Write;
use std::process::exit;
use crate::clay::parse::t2c;
use crate::clay::prelude::objects::func::{Func, Script};
use crate::clay::var::{ToVar, Virtual};
use crate::clay::vm::env::void_ctx;
use crate::clay::vm::signal::Signal;
use crate::clay::{parse, vm};

use super::objects::args::Args;
use super::objects::method::Method;

pub fn repl() -> Signal {
    println!("clay,启动\n输入exit退出");
    let vm = vm::runtime::Vm::new()?;

    inner_repl(Args::new(vm, &[] as &[_],void_ctx(vm)))
}

pub fn inner_repl(all: Args) -> Signal {
    let vm = all.vm().clone();
    let binding = vm.str()?.unbox()?;

    {
        
        let to_str: &Method = binding.cast()?;

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

            if input.trim() == "exit" {
                break;
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

            #[cfg(debug_assertions)]{
                println!("\n{:#?}\n", hc);
                continue;
            }
            
            let func = {
                let codes = t2c(&hc,all.vm())?;
                let script = Script::new(
                    &Some(if _begin == end{
                        format!("line {}",_begin)
                    }else{
                        format!("line {} ~ {}",_begin, end)
                    }),
                    &[], 
                    vm.get_context(),
                    &codes,
                );

                Func::Script(script).to_var(vm)
            };

            match vm.run_code(func) {
                Ok(v) => {
                    println!(
                        "{}\n",
                        match Virtual::call(to_str, Args::from((vm, &[v] as &[_]))) {
                            Ok(v) => match v.unbox() {
                                Ok(s_v) => match s_v.cast::<String>() {
                                    Ok(s) => s.to_string(),
                                    Err(e) => e.to_string(),
                                },
                                Err(e) => e.to_string(),
                            },
                            Err(e) => e.to_string(),
                        }
                    );
                }
                Err(e) => eprintln!("{}", e),
            };
        }

        exit(0);
    }
}
