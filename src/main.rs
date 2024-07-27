use clay::{
    parse,  vm::{self, error::VmError, signal::{Abort, ErrSignal}, Eval}
};

pub mod clay;

fn main() -> ErrSignal<()> {
    // println!("Hello, world!");
    // use clay::var::undef::test;
    // test();

    // let vm = clay::vm::Runtime::new();
    // vm.async_runtime().block_on(water());
    //prul

    let code = r#"
debug ''

puts '越过长城，走向世界' 

puts '' #试试注释

{
    puts (input '输入名字 ' )
    puts ''
    345
    '3456789876543'
    undef
}
"#;

    println!("{}", code);

    let hc = match parse::Parser::new(code).parse() {
        Ok(hc) => hc,
        Err(e) => {
            eprintln!("parse error: {}", e);
            return Err(
                Abort::ThrowError(Box::<VmError>::new("无法解析代码".into()))
            );
        }
    };

    //println!("\n{}",hc.to_string());
    println!("\n{:#?}\n", hc);

    {
        let vm = vm::Runtime::new()?;

        // Future::new(async move{
        //     vm.borrow().undef().unwrap()
        // }, vm);

        #[cfg(debug_assertions)]
        {
            println!("\nvm就绪");
        }

        #[cfg(debug_assertions)]
        {
            println!("\n主协程就绪");
        }

        #[cfg(debug_assertions)]
        {
            println!("\nundef就绪");

            println!("\n开始执行")
        }

        println!("{:#?}",hc.eval(vm, vm.borrow().get_context()));
    };

    Ok(())
}
