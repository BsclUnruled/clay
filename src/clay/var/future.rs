use crate::clay::vm::signal::Signal;

use crate::clay::vm::{error, runtime::Vm};

use super::ToVar;
pub use std::future::Future as StdFuture;

pub struct Future{
    //chan:Arc<tokio::sync::oneshot::Receiver<Signal>>
}

impl ToVar for Future{}

impl Future{
    pub fn new(task:impl StdFuture<Output=Signal> + Send +'static,vm:Vm) -> Self{
        let vm2 = vm.clone();
        vm.borrow().spawn(async move{
            let hc = vm2.borrow_mut();
            match hc.get_handle().recv().await{
                Some(_) => task.await,
                None => Err(error::async_scheduler_error())
            }
        });

        Self{}
    }
}