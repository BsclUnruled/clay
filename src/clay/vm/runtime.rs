use corosensei::{Coroutine, Yielder};
use std::{
    collections::{HashMap, LinkedList},
    rc::{Rc, Weak},
};

use crate::clay::{Cell,self, var::{func, undef, Cross, ToCross, VarBox}};

use super::{
    env::{self, Context}, signal::Signal
};

pub struct Runtime {
    //async_runtime:tokio::runtime::Runtime,
    id_stack: LinkedList<usize>,
    id_counter: usize,

    heap: LinkedList<Rc<VarBox>>,

    //undef:Cross,
    //lambda:Cross,
    mark_sweep: Coroutine<(), (), ()>,

    global_context: Rc<dyn Context>,
}

impl Runtime {
    pub fn get_context(&self)->Rc<dyn Context> {
        Rc::clone(&self.global_context)
    }
    pub fn get_id(&mut self) -> usize {
        match self.id_stack.pop_back() {
            Some(id) => id,
            None => {
                let id = self.id_counter;
                self.id_counter += 1;
                id
            }
        }
    }

    pub fn back_id(&mut self, id: usize) {
        self.id_stack.push_back(id);
    }

    pub fn push_heap(&mut self, var: VarBox) -> Weak<VarBox> {
        let strong = Rc::new(var);
        let weak = Rc::downgrade(&strong);
        self.heap.push_back(strong);
        weak
    }

    pub fn undef(&self) -> Signal {
        self.global_context.get("undef")
    }

    pub fn gc(&mut self) {
        match self.mark_sweep.resume(()) {
            corosensei::CoroutineResult::Yield(_) => (),
            corosensei::CoroutineResult::Return(_) => panic!("Error:gc协程退出"),
        }
    }

    pub fn new() -> Vm {
        let global = HashMap::new();

        let global_context = Rc::new((Cell::new(global), Some(env::undef_ctx())));

        let hc = Self {
            id_counter: 0,
            id_stack: LinkedList::new(),

            heap: LinkedList::new(),

            mark_sweep: Coroutine::new(Self::default_mark_sweep),

            global_context,
        };

        let hc = Box::leak(Box::new(Cell::new(hc)));

        Self::init(hc)
    }

    fn init(vm:Vm)->Vm {
        vm.borrow().def("undef",&undef::new(vm));

        {
            vm.borrow()
                .def(
                    "puts",
                    &func::Func::Native(
                        &clay::prelude::io::puts
                    ).to_cross(vm)
            );
            vm.borrow()
                .def(
                    "input",
                    &func::Func::Native(
                        &clay::prelude::io::input
                    ).to_cross(vm)
            );
        }

        vm
    }

    fn default_mark_sweep(ctrl: &Yielder<(), ()>, _: ()) {
        //todo
        // let step = 100;
        // let mut count = 0;

        loop {
            ctrl.suspend(())
        }
    }

    // pub fn lambda(&self)->Cross{
    //     self.lambda.clone()
    // }
}

impl Context for Runtime {
    fn def(&self, name: &str, value:&Cross) {
        self.global_context.def(name, value);
    }
    fn get(&self, name: &str)->Signal {
        self.global_context.get(name)
    }
    fn has(&self, name: &str)->bool {
        self.global_context.has(name)
    }
    fn set(&self, name: &str, value:&Cross) {
        self.global_context.set(name, value)
    }
}

pub type Vm = &'static Cell<Runtime>;
