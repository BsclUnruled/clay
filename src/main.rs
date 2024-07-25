//#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use clay::{
    parse,
    var::Cross,
    vm::{self, signal::Signal, Eval},
};
use corosensei::Coroutine;
use corosensei::Yielder;

pub mod clay;

fn main() -> Result<(), ()> {
    // println!("Hello, world!");
    // use clay::var::undef::test;
    // test();

    // let vm = clay::vm::Runtime::new();
    // vm.async_runtime().block_on(water());
    //prul

    let code = r#"
puts '越过长城，走向世界' 

puts '' #试试注释

{
    puts (input '输入名字 ' )
    puts ''

}

puts '程序结束'

"#;

    println!("{}", code);

    let hc = match parse::Parser::new(code).parse() {
        Ok(hc) => hc,
        Err(e) => {
            eprintln!("parse error: {}", e);
            return Err(());
        }
    };

    //println!("\n{}",hc.to_string());
    println!("\n{:?}\n", hc);

    {
        let vm = vm::Runtime::new();

        let mut coro = Coroutine::new(move |ctrl: &Yielder<Cross, Signal>, _| {
            //ctrl.suspend(vm.borrow().undef());

            hc.eval(vm, vm.borrow().get_context(), ctrl)
        });

        let undef = match vm.borrow().undef() {
            Ok(undef) => undef,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        match coro.resume(undef.clone()) {
            corosensei::CoroutineResult::Yield(val) => println!("\nyield: {:?}", val),
            corosensei::CoroutineResult::Return(val) => println!("\nreturn: {:?}", val),
        };
    };

    Ok(())
}
