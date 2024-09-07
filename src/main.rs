use std::fs /*io::Write*/;

use clay::{
    parse::{self, /*clay::Rule*/}, prelude::repl, var::Number, vm::signal::{Abort, ErrSignal}
};
// use pest::Parser;

pub mod clay;

fn clay_main() -> ErrSignal<()> {
    // println!("Hello, world!");
    // use clay::var::undef::test;
    // test();

    // let vm = clay::vm::Runtime::new();
    // vm.async_runtime().block_on(water());
    //prul

    let path: &str = &match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            return {
                repl::repl()?;
                Ok(())
            }
        }
    };

    let code = &match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("读取文件失败: {}", e);
            return Err(Abort::ThrowString("读取文件失败".into()));
        }
    };

    let hc = match parse::parse(code) {
        Ok(hc) => hc,
        Err(e) => {
            eprintln!("parse error: {}", e);
            return Err(Abort::ThrowString("无法解析代码".into()));
        }
    };

    //println!("\n{}",hc.to_string());

    {
        let vm = clay::vm::runtime::Vm::new()?;

        // Future::new(async move{
        //     vm.borrow().undef().unwrap()
        // }, vm);

        #[cfg(debug_assertions)]
        {
            println!("\nvm就绪");

            println!("{}", code);

            println!("\n{:#?}\n", hc);

            //输出程序所在路径
            println!("当前路径：{}", std::env::current_dir().unwrap().display());
            println!("程序路径：{}", path);

            println!("\nundef就绪");

            println!("\n开始执行\n")
        }

        println!("{:#?}", vm.undef()?.cast::<Number>()?);

        println!(
            "\n",
            // match vm.run_code(&(hc.format(&vm)?),) {
            //     Ok(v) => v,
            //     Err(e) => {
            //         match e {
            //             Abort::ThrowString(s) => eprintln!("Error: {}", s),
            //             _ => eprintln!("Error:不应出现的代码控制流 {:?}", e),
            //         }
            //         return Err(Abort::Exit);
            //     }
            // }
        );
    };

    Ok(())
}

fn main() {
//     let p = clay::parse::new_parser::ClayParser::parse;
//     println!("{:#?}",p(Rule::global,
// r#"
// for 

// if(12){ 
//     log (str 12)
// }else{
//     qq "wqd"
// }
// "#
//     ));
//     loop {
//         print!("\nclay> ");

//         //读取键盘输入
//         match std::io::stdout().flush() {
//             Ok(_) => (),
//             Err(e) => panic!("Error: {:?}", e),
//         };

//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();

//         if input.trim() == "exit" {break;}

//         let hc = 
//             clay::parse::new_parser::ClayParser::parse(Rule::global, &input);
    
//         println!("{:#?}\n",hc)
//     }

    println!("{:#?}", clay_main())
}
