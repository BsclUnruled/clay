use std::cell::RefCell;
use std::collections::LinkedList;
use crate::clay::var::VarBox;
use std::rc::{Weak, Rc};

#[derive(Clone,Copy)]
pub enum Mark{
    New,Marked,Unmarked,
}

static mut ID: usize = 0;

thread_local! {
    static ID_STACK:RefCell<LinkedList<usize>> = RefCell::new(LinkedList::new());
    static HEAP:RefCell<LinkedList<Rc<VarBox>>> = RefCell::new(LinkedList::new());
}

pub fn get_id()->usize{
    // if ID_STACK.with(|stack|{
    //     !stack.borrow().is_empty()
    // }){
    //     ID_STACK.with(|stack|{
    //         *stack.borrow_mut().back().unwrap()
    //     })
    // }else {
    //     unsafe {
    //         let id = ID;
    //         ID += 1;
    //         id
    //     }
    // }
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