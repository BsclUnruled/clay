//#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use clay::{
    parse,
    var::{undef, Cross},
};

pub mod clay;

fn main()->Result<(),()>{
    // println!("Hello, world!");
    // use clay::var::undef::test;
    // test();

    // let vm = clay::vm::Runtime::new();
    // vm.async_runtime().block_on(water());

    let code = 
r#"map it \(x){
    if {
        + x (randint 1 4)
    }else{
        x
    }
}"#;

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

async fn water() {
    println!("water");
}
