use std::cell::RefCell;
use std::collections::LinkedList;
use crate::clay::var::{Cross, VarBox};
use std::rc::{Weak, Rc};

#[derive(Clone,Copy)]
pub enum Mark{
    New,Marked,Unmarked,
}

static mut ID: usize = 0;

static mut HEAP_INDEX:usize = 0;

const STEP:usize = 100;

thread_local! {
    static ID_STACK:RefCell<LinkedList<usize>> = RefCell::new(LinkedList::new());
    static HEAP:RefCell<LinkedList<Rc<VarBox>>> = RefCell::new(LinkedList::new());
}

pub fn marker(var:&Cross){}

// #[coroutine]
// fn sweeper(){
//     let mut count = 0;
//     loop{
//         let mut new = LinkedList::new();
//         for var in HEAP.with(|heap|{heap.borrow().into_iter()}){
//             match var.get_mark(){
//                 Mark::New|Mark::Marked=>{
//                     var.set_mark(Mark::Unmarked);
//                     new.push_back(Rc::clone(&var));
//                 },
//                 Mark::Unmarked=>(),
//             }
//             count += 1;
//             if count >= STEP{
//                 count = 0;
//                 yield;
//             }
//         }

//         HEAP.with(|heap|{
//             *heap.borrow_mut() = new;
//         })
//     }
// }

// pub fn sweeper0(){
//     for index in unsafe{HEAP_INDEX}..(STEP + unsafe {HEAP_INDEX}){
//         match HEAP.with(|heap|{
//             let heap = heap.borrow();
//             let var = heap.get(index);
//             match var{
//                 Some(var)=>Some(
//                     Rc::clone(var)
//                 ),
//                 None=>None,
//             }
//         }){
//             Some(var)=>{
//                 match var.mark(){
//                     Mark::Marked=>{

//                     }
//                 }
//             },
//             None=>break
//         }
//     }
// }

pub fn get_id()->usize{
    match ID_STACK.with(|stack|{
        stack.borrow_mut().pop_back()
    }){
        Some(id) => id,
        None => unsafe {
            let id = ID;
            ID += 1;
            id
        }
    }
}

pub fn back_id(id:usize){
    ID_STACK.with(|stack|{
        stack.borrow_mut().push_back(id);
    });
}

pub fn push_heap(var:VarBox)->Weak<VarBox>{
    let rc = Rc::new(var);
    let weak = Rc::downgrade(&rc);
    HEAP.with(|heap|{
        heap.borrow_mut().push_back(rc);
    });
    weak
}