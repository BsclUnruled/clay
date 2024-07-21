#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

pub mod clay;

fn main() {
    println!("Hello, world!");
    use clay::var::undef::test;
    test();
}
