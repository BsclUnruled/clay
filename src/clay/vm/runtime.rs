// use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::{yield_now, JoinHandle}};

use super::{
    ctx, env::Env, heap::Heap, signal::{Abort, ErrSignal, Signal}, CtxType, ToRun
};
use crate::clay::{
    prelude::objects::undef,
    var::Var,
    Cell,
};
use std::{
    collections::LinkedList,
    ops::Deref,
    process::exit,
};

pub struct Runtime {
    //async_runtime:tokio::runtime::Runtime,
    id_stack: LinkedList<usize>,
    id_counter: usize,

    heap: Heap,

    global_context: Option<CtxType>,

    undef: Option<Var>,
    // ctrl:Control,
    // lock:Lock,

    // async_runtime:tokio::runtime::Runtime
}

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

pub trait Exit<T> {
    fn exit(&self, msg: T) -> !;
}

impl Exit<Abort> for Vm {
    fn exit(&self, msg: Abort) -> ! {
        eprint!("{}", msg);
        exit(1)
    }
}

impl Exit<&str> for Vm {
    fn exit(&self, msg: &str) -> ! {
        eprint!("{}", msg);
        exit(1)
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

impl Vm {
    // pub fn async_runtime(&self)->&tokio::runtime::Runtime{
    //     &self.borrow().async_runtime
    // }

    // pub fn run_code(&self,code:Var)->Signal{
    //     let vm = *self;
    //     self.async_runtime().spawn(async move{
    //         loop{
    //             let _ = vm.borrow_mut().ctrl.start_sender.blocking_send(());
    //             yield_now().await;
    //             match vm.borrow_mut().ctrl.finish_receiver.blocking_recv(){
    //                 Some(_) => (),
    //                 None =>{
    //                     #[cfg(debug_assertions)]{
    //                         eprintln!("未接收到数据(from Vm::run_code->eventloop)");
    //                     }
    //                 }
    //             };
    //         }
    //     });
    //     self.async_runtime().block_on(async{
    //         vm.borrow_mut().lock.start_receiver.recv().await.unwrap();
    //         let result =
    //             code.unbox()?.call(Args::new(
    //                 vm,
    //                 &[]
    //             ));
    //         let _ = vm.borrow_mut().lock.finish_sender.send(()).await;
    //         result
    //     })
    // }

    // pub fn spawn(&self,code:Var)->JoinHandle<Signal>{
    //     let vm = *self;
    //     self.async_runtime().spawn(async move{
    //         vm.borrow_mut().lock.start_receiver.recv().await.unwrap();
    //         let result =
    //             code.unbox()?.call(Args::new(
    //                 vm,
    //                 &[]
    //             ));
    //         let _ = vm.borrow_mut().lock.finish_sender.send(()).await;
    //         result
    //     })
    // }

    pub fn get_context(&self) -> &CtxType {
        match self.borrow().global_context {
            None => {
                self.borrow_mut().global_context = Some(ctx::default(*self, ctx::void_ctx(*self)));
                self.get_context()
            }
            Some(ref ctx) => ctx,
        }
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

    pub fn undef(&self) -> Signal {
        Ok(self
            .0
            .borrow()
            .undef
            .as_ref()
            .expect("undef被释放了")
            .clone())
    }

    pub fn new() -> ErrSignal<Vm> {
        // let (ctrl,lock) = make_lock();

        let hc = Runtime {
            id_counter: 1,
            id_stack: LinkedList::new(),

            heap: Heap::new(),

            global_context: None,

            undef: None,
            // ctrl,
            // lock,
        };

        let hc = Vm(Box::leak(Box::new(Cell::new(hc))));

        Self::init(hc)
    }

    pub fn run_code(&self, _func: ToRun) -> Signal {
        Err(Abort::ThrowString(
            "not implemented yet(Vm::run_code)".to_owned(),
        ))
    }

    fn init(vm: Vm) -> ErrSignal<Vm> {
        let env = Env::new(vm, vm.get_context().clone());

        vm.0.borrow_mut().undef = Some(
            vm.get_context()
                .def(&env, "undef", &undef::new(vm))
                .sync()
                .expect("undef载入失败"),
        );

        Ok(vm)
    }

    pub fn mut_heap(&self) -> &mut Heap {
        &mut self.0.borrow_mut().heap
    }

    pub fn heap(&self) -> &Heap {
        &self.borrow().heap
    }
}

unsafe impl Send for Vm {}
unsafe impl Sync for Vm {}
