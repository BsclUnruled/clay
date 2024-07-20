use tokio::time::sleep;
use tokio::time::Duration;

pub mod clay;

fn main() {
    println!("Hello, world!");
    use clay::var::undef::test;
    test();

    let size = 10;

    let mut stack = Vec::with_capacity(size);

    for i in 1..=size {
        stack.push(water(i as u64));
    }

    //全部等待
    let runtime = tokio::runtime::Runtime::new().expect("没能开启tokio");
    runtime.block_on(futures::future::join_all(stack));
}

async fn water(num:u64){
    sleep(Duration::from_secs(num)).await;
    println!("water function({})",num);
}
