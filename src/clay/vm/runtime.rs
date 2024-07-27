use crate::clay::{
    self,
    var::{func, future::StdFuture, undef, ToVar, Var, VarBox},
    Cell,
};
use std::{
    collections::{HashMap, LinkedList},
    ops::Deref,
    rc::{Rc, Weak},
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use super::{
    env::{self, Context},
    error::VmError,
    signal::{Abort, ErrSignal, Signal},
};

pub struct Runtime {
    //async_runtime:tokio::runtime::Runtime,
    id_stack: LinkedList<usize>,
    id_counter: usize,

    heap: LinkedList<Rc<VarBox>>,

    //undef:Cross,
    //lambda:Cross,
    async_runtime: tokio::runtime::Runtime,

    scheduler: Sender<()>,
    handle: Receiver<()>,

    global_context: Rc<dyn Context>,
}

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

impl Runtime {
    pub fn get_context(&self) -> Rc<dyn Context> {
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

    pub fn async_runtime(&self) -> &tokio::runtime::Runtime {
        &self.async_runtime
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

    pub fn new() -> ErrSignal<Vm> {
        let global = HashMap::new();

        let global_context = Rc::new((Cell::new(global), Some(env::undef_ctx())));

        let (scheduler, handle) = channel(1);

        let hc = Self {
            id_counter: 0,
            id_stack: LinkedList::new(),

            heap: LinkedList::new(),

            async_runtime: match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => return Err(Abort::ThrowError(Box::new(e))),
            },
            scheduler,
            handle,

            global_context,
        };

        let hc = Vm(Box::leak(Box::new(Cell::new(hc))));

        Ok(Self::init(hc))
    }

    fn init(vm: Vm) -> Vm {
        vm.borrow().def("undef", &undef::new(vm));

        {
            vm.borrow().def(
                "puts",
                &func::Func::Native(&clay::prelude::io::puts, "puts".into()).to_var(vm),
            );

            vm.borrow().def(
                "input",
                &func::Func::Native(&clay::prelude::io::input, "input".into()).to_var(vm),
            );

            vm.borrow().def(
                "debug",
                &func::Func::Native(&clay::prelude::debug::debug, "debug".into()).to_var(vm),
            );

            vm.borrow().def(
                "add",
                &func::Func::Native(&clay::prelude::math::add, "add".into()).to_var(vm),
            );
        }

        vm
    }

    pub fn get_handle(&mut self) -> &mut Receiver<()> {
        &mut self.handle
    }

    pub fn schedule(&mut self) -> ErrSignal<()> {
        match self.scheduler.try_send(()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Abort::ThrowError(
                VmError::new(
                    &format!("Vm@{:?}进行clay异步函数调度失败", self as *const Runtime),
                    Some(Box::new(e)),
                )
                .into(),
            )),
        }
    }

    pub fn spawn(&self, task: impl StdFuture<Output = Signal> + Send + 'static) {
        self.async_runtime.spawn(task);
    }

    // pub fn lambda(&self)->Cross{
    //     self.lambda.clone()
    // }
}

impl Context for Runtime {
    fn def(&self, name: &str, value: &Var) {
        self.global_context.def(name, value);
    }
    fn get(&self, name: &str) -> Signal {
        self.global_context.get(name)
    }
    fn has(&self, name: &str) -> bool {
        self.global_context.has(name)
    }
    fn set(&self, name: &str, value: &Var) {
        self.global_context.set(name, value)
    }
}

#[derive(Clone, Copy)]
pub struct Vm(&'static Cell<Runtime>);

impl Deref for Vm {
    type Target = Cell<Runtime>;
    fn deref(&self) -> &'static Self::Target {
        &self.0
    }
}

unsafe impl Send for Vm {}
unsafe impl Sync for Vm {}
