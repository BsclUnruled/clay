use std::{cell::RefCell, collections::LinkedList, rc::{Rc, Weak}};
use corosensei::{Coroutine, Yielder};

use crate::clay::var::{undef, Cross, VarBox};

pub struct Runtime {
    //async_runtime:tokio::runtime::Runtime,
    id_stack:LinkedList<usize>,
    id_counter:usize,

    heap:LinkedList<Rc<VarBox>>,

    undef:Cross,
    //lambda:Cross,

    mark_sweep:Coroutine<(),(),()>
}

impl Runtime {
    pub fn get_id(&mut self)->usize{
        match self.id_stack.pop_back() {
            Some(id) => id,
            None => {
                let id = self.id_counter;
                self.id_counter += 1;
                id
            }
        }
    }

    pub fn back_id(&mut self, id:usize){
        self.id_stack.push_back(id);
    }

    pub fn push_heap(&mut self, var:VarBox)->Weak<VarBox>{
        let strong = Rc::new(var);
        let weak = Rc::downgrade(&strong);
        self.heap.push_back(strong);
        weak
    }

    pub fn undef(&self)->Cross{
        self.undef.clone()
    }

    pub fn gc(&mut self){
        match self.mark_sweep.resume(()){
            corosensei::CoroutineResult::Yield(_)=>(),
            corosensei::CoroutineResult::Return(_)=>panic!(
                "Error:gc协程退出"
            ),
        }
    }

    pub fn new()->Self{
        let mut heap = LinkedList::new();
        let undef = undef::init(&mut heap);

        Self{
            id_counter:1,//undef用去0
            id_stack:LinkedList::new(),

            heap,
            undef,

            mark_sweep:Coroutine::new(
                Self::default_mark_sweep,
            )
        }
    }

    fn default_mark_sweep(
        ctrl:&Yielder<(),()>,
        _:(),
    ){//todo
        // let step = 100;
        // let mut count = 0;

        loop{
            ctrl.suspend(())
        }
    }

    pub fn new_vm()->Vm{
        Box::leak(
            Box::new(
                RefCell::new(
                    Self::new()
                )
            )
        )
    }

    // pub fn lambda(&self)->Cross{
    //     self.lambda.clone()
    // }
}

pub type Vm = &'static RefCell<Runtime>;