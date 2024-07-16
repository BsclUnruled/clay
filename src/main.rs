pub mod clay;

fn main() {
    println!("Hello, world!");
    let binding = clay::var::undef::undef();
    let binding = binding.uncross();
    let func:Option<&clay::var::func::Func> = binding.cast();//会painc(段错误 (核心已转储))
    println!("{:?}", func);
}
