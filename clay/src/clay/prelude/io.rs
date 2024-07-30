use crate::clay::{
    var::ToVar,
    vm::signal::Signal,
};
use std::io::{BufRead, Write};
use super::objects::func::Args;

pub fn puts(args: Args) -> Signal {
    let (vm, args, _) = args;
    match args.get(0) {
        Some(msg) => {
            println!("{}", msg.unbox()?.to_string());
            Ok(vm.undef()?)
        }
        None => {
            println!();
            Ok(vm.undef()?)
        }
    }
}

pub fn input(args: Args) -> Signal {
    let (vm, args, _) = args;
    match args.get(0) {
        Some(msg) => Ok(inner_input(msg.unbox()?.to_string()).to_var(vm)),
        None => Ok(inner_input("".to_owned()).to_var(vm)),
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
