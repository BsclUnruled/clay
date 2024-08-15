use super::{runtime::Vm, CtxType};
use crate::clay::var::Var;
use std::{collections::LinkedList, rc::Rc};

#[derive(Clone, Copy, Debug)]
pub enum Mark {
    New,
    Marked,
    Unmarked,
}

pub fn gc(root: &CtxType, vm: Vm) {
    while !vm.heap().is_empty(){
        root.set_mark(Mark::New);
        ms(root, vm);
    }
}

fn ms(root: &CtxType, vm: Vm) {
    root.gc_for_each(marker);

    let mut async_iter = vm.heap().iter();

    let mut heap = LinkedList::new();

    #[cfg(debug_assertions)]
    {
        //println!("Gc状态: Sweeping({:?})", mark);
    }

    while let Some(var) = async_iter.next() {
        //ctrl.suspend(());

        let mark = var.get_mark();

        #[cfg(debug_assertions)]
        {
            println!("Gc状态: Sweeping({:?})", mark);
        }

        match mark {
            Mark::Unmarked => {
                #[cfg(debug_assertions)]
                {
                    println!("Gc状态: 释放变量({:?})", var.get_id());
                }
            }
            _ => {
                #[cfg(debug_assertions)]
                {
                    println!("Gc状态: 整理堆({:?})", mark);
                }

                heap.push_back(Rc::clone(&var));
                var.set_mark(Mark::Unmarked);
            }
        }
    }

    *vm.mut_heap() = heap;
}

fn marker(var: &Var) {
    #[cfg(debug_assertions)]
    {
        println!("Gc状态: Marking");
    }

    match var.unbox() {
        Ok(v) => match v.get_mark() {
            Mark::New | Mark::Unmarked => {
                v.set_mark(Mark::Marked);
                v.gc_for_each(marker);
            }
            Mark::Marked => {}
        },
        Err(e) => panic!("{}", e.as_string()),
    }
}
