use clay::{
    parse, var::func::Func, vm::{self, env::Context, error::VmError, signal::{Abort, ErrSignal}, Eval}
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
    
}

puts (add 213 342)
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
            println!("\nundef就绪");

            println!("\n开始执行\n")
        }

        #[cfg(debug_assertions)]{
            let hc = vm.borrow().get("debug");

            println!("{:?}",hc);

            let hc = hc?.unbox();

            println!("{:?}",hc);

            let hc = hc?;

            println!("{:?}",*hc);
            println!("is Func?: {}",hc.is::<crate::Func>());

            let hc:Option<&Func> = hc.cast();

            println!("{:?}",hc);

            match hc{
                Some(f)=>println!("{:?}",f as *const _),
                None=>eprintln!("不存在"),
            }

            // let hc2 = &5 as &dyn std::any::Any;

            // println!("{:?}",hc2);
            // println!("is Int?: {}",hc2.is::<i32>());
            // println!("{:?}",hc2.downcast_ref::<i32>().unwrap() as *const _);
            // println!("{}",hc2.downcast_ref::<i32>().unwrap());
        }

        println!("\n{:#?}",match hc.format().eval(vm, vm.borrow().get_context()){
            Ok(v) => v,
            Err(e) => {
                match e {
                    Abort::ThrowError(e)=>eprintln!("{}",e),
                    Abort::ThrowString(s)=>eprintln!("Error: {}",s),
                    _=>eprintln!("Error:不应出现的代码控制流 {:?}",e),
                }
                return Err(
                    Abort::Exit
                );
            }
        });
    };

    Ok(())
}
