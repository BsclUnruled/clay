use std::{collections::LinkedList, rc::Rc};
use tokio::task::yield_now;
use tokio_stream::{self as stream, StreamExt};
use crate::clay::var::Var;
use super::runtime::Vm;

#[derive(Clone,Copy,Debug)]
pub enum Mark{
    New,Marked,Unmarked,
}

pub async fn gc(root:&Var,vm:Vm){
    #[cfg(debug_assertions)]
        let mut gc_count = 0;
    

    loop{
        #[cfg(debug_assertions)]{
            println!("Gc状态: 新一轮gc({})开始",gc_count);
            gc_count += 1;
        }

        match vm.gc_closer().try_recv(){
            Ok(_)=>{
                #[cfg(debug_assertions)]{
                    println!("Gc状态: 关闭gc");
                }
                break;
            }

            Err(_)=>{}
        };

        ms(root,vm).await;
    }
}

async fn ms(root:&Var,vm:Vm){
    root.unbox()
        .expect("Error: Gc启动失败,根变量已经释放")
        .for_each(marker);

    let mut async_iter = stream::iter(
        vm.heap()
    );

    let mut heap = LinkedList::new();

    #[cfg(debug_assertions)]{
        //println!("Gc状态: Sweeping({:?})", mark);
    }

    while let Some(var) = async_iter.next().await {
        yield_now().await;

        let mark = var.get_mark();

        #[cfg(debug_assertions)]{
            println!("Gc状态: Sweeping({:?})", mark);
        }

        match mark{
            Mark::Unmarked=>{
                #[cfg(debug_assertions)]{
                    println!("Gc状态: 释放变量({:?})",var);
                }
            },
            _=> {
                #[cfg(debug_assertions)]{
                    println!("Gc状态: 整理堆({:?})",mark);
                }

                heap.push_back(Rc::clone(&var));
                var.set_mark(Mark::Unmarked);
            },
        }
    }

    *vm.mut_heap() = heap;
}

fn marker(var:&Var){
    #[cfg(debug_assertions)]{
        println!("Gc状态: Marking");
    }

    match var.unbox(){
        Ok(v) => match v.get_mark(){
            Mark::New|Mark::Unmarked => {
                v.set_mark(Mark::Marked);
                v.for_each(marker);
            }
            Mark::Marked => {}
        }
        Err(e) => println!("{:?}",e)
    }
}