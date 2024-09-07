use std::collections::LinkedList;

use crate::clay::var::VarPtr;

pub struct Heap(LinkedList<*mut VarPtr>);

impl Heap {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    pub fn alloc(&mut self, var:VarPtr)->*mut VarPtr{
        let ptr = Box::into_raw(Box::new(var));
        self.0.push_back(ptr);
        ptr
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mark{
    Marked,
    Unmarked,
    New
}