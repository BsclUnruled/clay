use std::{cell::Cell, collections::LinkedList, rc::Rc};

use super::{ToCross, VarBox};
use crate::clay::{var::Cross, vm::{self, runtime::Vm}};

#[derive(Debug)]
struct Undef();

impl ToCross for Undef {
    fn to_cross(self,_:Vm) -> super::Cross {
        // Cross::new(
        //     Box::new(self),vm
        // )

        panic!("Error: 使用ToCross创建undef")
    }
}

pub fn init(
    heap:&mut LinkedList<Rc<VarBox>>//vm的heap,先行交给undef::init来初始化undef
    ) -> super::Cross {
    let undef = Rc::new(VarBox{
        mark:Cell::new(vm::gc::Mark::New),
        id:0usize,
        value: Box::new(Undef()),
    });

    let result = Cross { 
        weak: Rc::downgrade(&undef)
    };
    
    heap.push_back(undef);

    result
}

// pub fn test() {
//     let ud = undef();
//     let ud2 = ud.uncross();
//     let ud3 = ud2.cast::<Undef>();
//     println!("{:?}", ud3);
// }