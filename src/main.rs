pub mod clay;

fn main() {
    println!("Hello, world!");
    let binding = clay::var::undef::undef();
    let ud:&clay::var::undef::Undef = binding.cast();
    println!("{:?}", ud);
    let binding = clay::var::undef::undef();
    let func:&clay::var::func::Func = binding.cast();//会painc(段错误 (核心已转储))
    println!("{:?}", func);
}
