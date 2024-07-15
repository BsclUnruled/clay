use std::cell::RefCell;
use std::collections::LinkedList;

#[derive(Clone,Copy)]
pub enum Mark{
    New,Marked,Unmarked,
}

static mut ID: usize = 0;

thread_local! {
    static ID_STACK:RefCell<LinkedList<usize>> = RefCell::new(LinkedList::new());
}

pub fn get_id()->usize{
    if ID_STACK.with(|stack|{
        !stack.borrow().is_empty()
    }){
        ID_STACK.with(|stack|{
            *stack.borrow_mut().back().unwrap()
        })
    }else {
        unsafe {
            let id = ID;
            ID += 1;
            id
        }
    }
}