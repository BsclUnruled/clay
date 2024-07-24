//#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use clay::parse;

pub mod clay;

fn main()->Result<(),()>{
    // println!("Hello, world!");
    // use clay::var::undef::test;
    // test();

    // let vm = clay::vm::Runtime::new();
    // vm.async_runtime().block_on(water());

    let code = r#"
map arr \(x){
    log 'rty'

    if(eq x 1){
        0.9
    }else{
        -114
        -19.7
    }


}    
"#;

    println!("{}",code);

    let hc = match parse::Parser::new(code).parse(){
        Ok(hc) => hc,
        Err(e) => {
            println!("parse error: {}", e);
            return Err(());
        }
    };

    //println!("\n{}",hc.to_string());
    println!("\n{:?}",hc);
    Ok(())
}
