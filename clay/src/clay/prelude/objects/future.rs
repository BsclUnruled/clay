use crate::clay::vm::signal::Signal;

use crate::clay::vm::{error, runtime::Vm};

use super::func::Args;
use crate::clay::var::Virtual;
use std::fmt::Display;
pub use std::future::Future as StdFuture;

#[derive(Debug)]
pub struct Future{}

impl Display for Future{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Future@{:p}",self)
    }
}

impl Virtual for Future{
    fn as_func(&self,_:Args)->Signal
    where Self:Sized + 'static{
        Err(
            error::not_a_func()
        )
    }
}

impl Future{
    pub fn new(task:impl StdFuture<Output=Signal> + Send +'static,vm:Vm) -> Self{
        vm.spawn(async move{
            match vm.schedule().recv().await{
                Some(_) => task.await,
                None => Err(error::async_scheduler_error())
            }
        });

        Self{}
    }
}