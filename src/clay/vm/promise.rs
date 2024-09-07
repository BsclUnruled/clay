use std::{future::Future, pin::Pin, sync::Arc, task::{Context, Poll, Wake, Waker}};

use crate::clay::var::Var;

use super::{env::Env, signal::Signal};
use std::cell::UnsafeCell as UCell;

static mut CTX: UCell<Option<Context<'static>>> = UCell::new(None);

pub fn get_void<'f>() -> &'f mut Context<'static> {
    let refer = unsafe { &mut *CTX.get() };
    if let Some(value) = refer.as_mut() {
        value
    } else {
        let value = init();
        let refer = unsafe { &mut *CTX.get() };
        *refer = Some(value);
        refer
            .as_mut()
            .expect("Error: 无法在初始化后获取值(from clay::Cell::get_mut_or_init)")
    }
}

fn init() -> Context<'static> {
    Context::from_waker(Box::leak(Box::new(Waker::from(Arc::new(Void())))))
}

struct Void();

impl Wake for Void {
    fn wake(self: Arc<Self>) {}
}

pub struct Promise {
    future: Pin<Box<dyn Future<Output = Signal>>>,
}

impl Future for Promise {
    type Output = Signal;

    fn poll(mut self:Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        self.future.as_mut().poll(cx)
    }
}

impl Promise {
    pub fn sync(mut self) -> Signal {
        loop{
            match self.future.as_mut().poll(get_void()) {
                Poll::Ready(signal) => return signal,
                Poll::Pending => {}
            }
        }
    }

    pub fn get(self,env:&Env,name: &str)->Self{
        let env = env.clone();
        let name = name.to_owned();
        promise(async move{
            let var = self.await?;
            var.get(&env,&name).await
        })
    }

    pub fn call(self,env:&Env,args:&[Var])->Self{
        let env = env.clone();
        let args = args.to_owned();
        promise(async move{
            let var = self.await?;
            var.call(&env,&args).await
        })
    }
}

impl From<Signal> for Promise {
    fn from(value: Signal) -> Self {
        resolve(value)
    }
}

pub fn promise<F>(future: F) -> Promise
where
    F: Future<Output = Signal> + 'static,
{
    Promise {
        future: Box::pin(future),
    }
}

pub fn resolve(signal: Signal) -> Promise {
    promise(async move { signal })
}