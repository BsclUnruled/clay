// use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::{yield_now, JoinHandle}};

use crate::clay::{
    prelude::{objects::{ method, undef}, to_str}, var::{Var, VarBox,}, Cell
};
use std::{
    collections::LinkedList, ops::Deref, process::exit, rc::{Rc, Weak}
};
use super::{
    env, signal::{Abort, ErrSignal, Signal}, CtxType
};

pub struct Runtime {
    //async_runtime:tokio::runtime::Runtime,
    id_stack: LinkedList<usize>,
    id_counter: usize,

    heap: LinkedList<Rc<VarBox>>,

    global_context: Option<CtxType>,

    undef: Option<Var>,
    r#str:Option<Var>,

    // ctrl:Control,
    // lock:Lock,

    // async_runtime:tokio::runtime::Runtime
}

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

pub trait Exit<T>{
    fn exit(&self,msg:T)->!;
}

impl Exit<Abort> for Vm{
    fn exit(&self,msg:Abort)->! {
        eprint!("{}",msg);
        exit(1)
    }
}

impl Exit<&str> for Vm{
    fn exit(&self,msg:&str)->! {
        eprint!("{}",msg);
        exit(1)
    }
}

// pub struct InnerLock{
//     start_receiver:Receiver<()>,
//     finish_sender:Sender<()>
// }

// #[derive(Clone)]
// pub struct Lock{
//     inner:Rc<Cell<InnerLock>>
// }

// impl Deref for Lock {
//     type Target = InnerLock;
//     fn deref(&self) -> &Self::Target {
//         self.inner.borrow()
//     }
// }

// impl DerefMut for Lock {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.inner.borrow_mut()
//     }
// }

// impl Virtual for Lock {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
// }

// pub struct Control{
//     start_sender:Sender<()>,
//     finish_receiver:Receiver<()>
// }

// pub fn make_lock()->(Control,Lock){
//     let (start_sender,start_receiver) = channel(1);
//     let (finish_sender,finish_receiver) = channel(1);
//     (
//         Control{start_sender,finish_receiver},
//         Lock{
//             inner:Rc::new(Cell::new(InnerLock{
//                 start_receiver,
//                 finish_sender
//             }))
//         }
//     )
// }

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

    pub fn get_context(&self) ->&CtxType {
        match self.borrow().global_context{
            None =>{
                self.borrow_mut().global_context = Some(
                    env::default(*self,env::void_ctx(*self))
                );
                self.get_context()
            }
            Some(ref ctx) => ctx
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
        Ok(self.0.borrow().undef.as_ref().expect("undef被释放了").clone())
    }

    pub fn r#str(&self)->Signal{
        Ok(self.0.borrow().r#str.as_ref().expect("r#str被释放了").clone())
    }

    pub fn new() -> ErrSignal<Vm> {

        // let (ctrl,lock) = make_lock();

        let hc = Runtime {
            id_counter: 1,
            id_stack: LinkedList::new(),

            heap: LinkedList::new(),

            global_context:None,

            undef: None,
            r#str: None,

            // ctrl,
            // lock,
        };

        let hc = Vm(Box::leak(Box::new(Cell::new(hc))));

        Ok(Self::init(hc))
    }

    pub fn run_code(&self,_func:Var)->Signal{
        Err(Abort::ThrowString("not implemented yet(Vm::run_code)".to_owned()))
    }

    fn init(vm: Vm) -> Vm {
        
                
        vm.0.borrow_mut().undef = Some(
            vm.get_context().def(vm,"undef", &undef::new(vm))
                .expect("undef载入失败")
        );

        /*global_init*/{
            method::global_init(vm);
            to_str::global_init(vm);    
        }

        vm.0.borrow_mut().r#str = Some(
            vm.get_context().get(vm,"str")
                .expect("str载入失败")
        );

        vm
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
