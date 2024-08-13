use super::args::Args;
use crate::clay::var::{Var, Virtual};
use crate::clay::vm::signal::{Abort, Signal};
use crate::clay::Cell;
use std::ops::Deref;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use std::{future::Future as StdFuture, pin::Pin, task::Waker};

static mut CTX: Cell<Option<Context<'static>>> = Cell::none();
//Context::from_waker(Box::leak(Box::new(Waker::from(Arc::new(Void())))))
struct Void();

impl Wake for Void {
    fn wake(self: Arc<Self>) {}
}

pub struct Inner<F: StdFuture<Output = Signal>> {
    future: Pin<Box<F>>,
    done: bool,
    sender: Sender<Var>,
    receiver: Receiver<Signal>,
}

pub struct Future<F: StdFuture<Output = Signal>> {
    inner: Cell<Inner<F>>,
}

impl<F: StdFuture<Output = Signal>> Future<F> {
    pub fn form_future(future: F, sender: Sender<Var>, receiver: Receiver<Signal>) -> Self {
        let hc = Inner {
            future: Box::pin(future),
            done: false,
            sender,
            receiver,
        };
        Self {
            inner: Cell::new(hc),
        }
    }
}

impl<F: StdFuture<Output = Signal>> Deref for Future<F> {
    type Target = Cell<Inner<F>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<F: StdFuture<Output = Signal> + 'static> Virtual for Future<F> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn call(&self, all: Args) -> Signal {
        if self.borrow().done {
            return all.vm().undef();
        } else {
            let result = self.borrow_mut().sender.send(match all.args().get(0) {
                Some(v) => v.clone(),
                None => all.vm().undef()?,
            });
            match result {
                _ => (),
            };
            match self.borrow_mut().future.as_mut().poll(get_ctx()) {
                Poll::Ready(val) => val,
                Poll::Pending => match self.borrow_mut().receiver.try_recv() {
                    Ok(val) => val,
                    Err(e) => Err(Abort::ThrowString(e.to_string())),
                },
            }
        }
    }
}

pub enum Step<Y, R> {
    Yield(Y),
    Return(R),
}

fn get_ctx<'f>() -> &'f mut Context<'static> {
    unsafe { CTX.get_mut_or_init(init) }
}

fn init() -> Context<'static> {
    Context::from_waker(Box::leak(Box::new(Waker::from(Arc::new(Void())))))
}

#[macro_export]
macro_rules! co {
    //表示匹配块
    ($sender:ident,$receiver:ident,$body:expr) => {{
        use stackful::stackful;
        let (send_to_inner, $receiver) = std::sync::mpsc::channel();
        let ($sender, receive_from_inner) = std::sync::mpsc::channel();
        let hc = stackful($body);
        Future::form_future(hc, send_to_inner, receive_from_inner)
    }};
}

// fn test() {
//     co!(_se,re,move || {
//         Ok(re.try_recv().unwrap())
//     });
// }
