use crate::clay::{
    self, prelude::objects::{func::Func, undef}, var::{self, ToVar, Var, VarBox}, Cell
};
use std::{
    collections::LinkedList, ops::Deref, rc::{Rc, Weak}
};
use futures::FutureExt;
use tokio::sync::mpsc::{channel, error::TrySendError, Receiver, Sender};
use crate::clay::prelude::objects::future::StdFuture;
use super::{
    gc::gc, signal::{Abort, ErrSignal, Signal}, Code, Eval
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

    gc_closer:Receiver<()>,
    gc_teller:Sender<()>,

    global_context: Rc<VarBox>,
    root: Var,
}

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

#[derive(Clone, Copy)]
pub struct Vm(&'static Cell<Runtime>);

impl Deref for Vm {
    type Target = Cell<Runtime>;
    fn deref(&self) -> &'static Self::Target {
        &self.0
    }
}

impl Vm {
    pub fn run_code(&self,code:&Code)->Signal{
        let ctx = self.get_context();

        // self.borrow().async_runtime.spawn();

        self.async_runtime().block_on(async{
            let gc = gc(self.root(), *self);
            let task = async {
                match self.borrow().scheduler.try_send(()) {
                    Ok(_) => code.eval(*self,ctx),
                    Err(e) => panic!("Vm@{:p}进行clay异步函数调度失败\n\t{}", self,e)
                }
            };

            futures::select!{
                _ = gc.fuse() => Err(Abort::ThrowString("Gc退出".to_owned())),
                result = task.fuse() => {
                    self.borrow().gc_teller.try_send(()).expect("无法通知Gc退出");
                    result
                }
            }
        })
    }

    pub fn gc_closer(&self) -> &mut Receiver<()> {
        &mut self.borrow_mut().gc_closer
    }

    pub fn get_context(&self) -> Rc<VarBox> {
        Rc::clone(&self.borrow().global_context)
    }
    pub fn get_id(&self) -> usize {
        let inner = self.borrow_mut();
        match inner.id_stack.pop_back() {
            Some(id) => id,
            None => {
                let id = inner.id_counter;
                inner.id_counter += 1;
                id
            }
        }
    }

    pub fn async_runtime(&self) -> &tokio::runtime::Runtime {
        &self.borrow().async_runtime
    }

    pub fn back_id(&self, id: usize) {
        self.borrow_mut().id_stack.push_back(id);
    }

    pub fn push_heap(&self, var: VarBox) -> Weak<VarBox> {
        let strong = Rc::new(var);
        let weak = Rc::downgrade(&strong);
        self.borrow_mut().heap.push_back(strong);
        weak
    }

    pub fn undef(&self) -> Signal {
        self.borrow().global_context.get(*self,"undef")
    }

    pub fn root(&self)->&Var{
        &self.borrow().root
    }

    pub fn new() -> ErrSignal<Vm> {
        

        let global_context = 
            Rc::new(var::VarBox::global_context());

        let root = Var{weak:Rc::downgrade(&global_context)};
            

        let (scheduler, handle) = channel(size_of::<()>() * 1 + 1);

        let (gc_teller,gc_closer) = channel(size_of::<()>() * 1 + 1);

        let hc = Runtime {
            id_counter: 1,
            id_stack: LinkedList::new(),

            heap: LinkedList::new(),

            async_runtime: match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => return Err(Abort::ThrowError(Box::new(e))),
            },
            scheduler,
            handle,

            gc_closer,
            gc_teller,

            root,

            global_context,
        };

        let hc = Vm(Box::leak(Box::new(Cell::new(hc))));

        Ok(Self::init(hc))
    }

    fn init(vm: Vm) -> Vm {
        let _ = vm.borrow().global_context.def(vm,"undef", &undef::new(vm));

        {
            let _ = vm.borrow().global_context.def(vm,
                "puts",
                &Func::Native(&clay::prelude::io::puts, "puts".into()).to_var(vm),
            );

            let _ = vm.borrow().global_context.def(vm,
                "input",
                &Func::Native(&clay::prelude::io::input, "input".into()).to_var(vm),
            );

            let _ = vm.borrow().global_context.def(vm,
                "add",
                &Func::Native(&clay::prelude::math::add, "add".into()).to_var(vm),
            );
        }

        vm
    }

    pub fn schedule<'f>(&self)
        -> &mut Receiver<()>{
        match self.borrow().scheduler.try_send(()) {
            Ok(_) => &mut self.borrow_mut().handle,
            Err(e) => match e {
                TrySendError::Full(_) => &mut self.borrow_mut().handle,
                TrySendError::Closed(_) => panic!("Vm@{:p}进行clay异步函数调度失败\n\t{}", self,e)
            }
        }
    }

    pub fn spawn(&self, task: impl StdFuture<Output = Signal> + Send + 'static) {
        self.async_runtime().spawn(task);
    }

    pub fn mut_heap(&self) -> &mut LinkedList<Rc<VarBox>> {
        &mut self.0.borrow_mut().heap
    }

    pub fn heap(&self) -> &LinkedList<Rc<VarBox>> {
        &self.borrow().heap
    }
}

unsafe impl Send for Vm {}
unsafe impl Sync for Vm {}
