//#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use clay::parse;
use corosensei::{Coroutine, CoroutineResult};

pub mod clay;

fn main()->Result<(),()>{
    // println!("Hello, world!");
    // use clay::var::undef::test;
    // test();

    // let vm = clay::vm::Runtime::new();
    // vm.async_runtime().block_on(water());

    let code = r#"
map arr \(x){
    eq x 1
}    
"#;

    let hc = match parse::Parser::new(code).parse(){
        Ok(hc) => hc,
        Err(e) => {
            println!("parse error: {}", e);
            return Err(());
        }
    };

    println!("{}",hc.to_string());
    println!("{:?}",hc);
    Ok(())
}
