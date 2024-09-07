use crate::clay::var::Var;
use crate::clay::vm::env::Env;
use crate::clay::vm::signal::Signal;
use crate::clay::{parse, vm};
use std::io::Write;
use std::process::exit;

pub fn repl() -> Signal {
    println!("clay,启动\n输入.exit退出");
    let vm = vm::runtime::Vm::new()?;

    inner_repl(&Env::new(vm,vm.get_context().clone()), &[])
}

pub fn inner_repl(env:&Env, _:&[Var]) -> Signal {
    let _vm = *env.vm();

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

            let hc = match parse::parse(&input) {
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

            match _vm.run_code(hc) {
                Ok(v) => {
                    println!(
                        "{}\n",
                        v.get(env, "toStr")
                            .call(env,&[])
                            .sync()?
                            .cast::<String>()?
                    );
                }
                Err(e) => eprintln!("{}", e),
            };
        }

        exit(0);
    }
}
