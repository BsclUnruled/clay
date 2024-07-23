extern crate context;
extern crate libc;

use std::iter::Iterator;
use std::mem::transmute;
use std::cell::{Cell, UnsafeCell};
use std::default::Default;
use std::ops::DerefMut;
use std::fmt;
use std::rt::unwind::try;
use std::rt::unwind::begin_unwind;
use std::rt;
use std::boxed::FnBox;
use std::any::Any;
use std::ptr;

use context::Context;
use context::stack::{Stack, StackPool};

use options::Options;
use {Result, Error};

thread_local!(static STACK_POOL: UnsafeCell<StackPool> = UnsafeCell::new(StackPool::new()));

struct ForceUnwind;

/// Initialization function for make context
extern "C" fn coroutine_initialize(_: usize, f: *mut libc::c_void) -> ! {
    {
        let func: Box<Box<FnBox()>> = unsafe { Box::from_raw(f as *mut Box<FnBox()>) };

        func();
    }

    unreachable!("Never reach here");
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Created,
    Running,
    Finished,
    ForceUnwind,
}

#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct SymmetricCoroutine<T = ()>
    where T: Send
{
    parent: Cell<*mut SymmetricCoroutine<T>>,
    context: Context,
    stack: Option<Stack>,

    name: Option<String>,
    state: State,

    result: Option<Result<*mut Option<T>>>,
}

unsafe impl<T> Send for SymmetricCoroutine<T> where T: Send {}

impl<T> fmt::Display for SymmetricCoroutine<T>
    where T: Send
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coroutine({})", self.name.as_ref()
                                            .map(|s| &s[..])
                                            .unwrap_or("<unnamed>"))
    }
}

impl<T> SymmetricCoroutine<T>
    where T: Send
{
    unsafe fn empty() -> Box<SymmetricCoroutine<T>> {
        Box::new(SymmetricCoroutine {
            parent: Cell::new(ptr::null_mut()),
            context: Context::empty(),
            stack: None,
            name: None,
            state: State::Created,
            result: None,
        })
    }

    pub fn spawn_opts<F>(f: F, opts: Options) -> Box<SymmetricCoroutine<T>>
        where F: FnOnce(SymmetricCoroutineRef<T>)
    {
        let mut stack =
            STACK_POOL.with(|pool| unsafe { (&mut *pool.get()).take_stack(opts.stack_size) });

        let mut coro = Box::new(SymmetricCoroutine {
            parent: Cell::new(ptr::null_mut()),
            context: Context::empty(),
            stack: None,
            name: opts.name,
            state: State::Created,
            result: None,
        });

        let coro_ref: &mut SymmetricCoroutine<T> = unsafe {
            let ptr: *mut SymmetricCoroutine<T> = coro.deref_mut();
            &mut *ptr
        };

        let puller_ref = SymmetricCoroutineRef { coro: coro_ref };

        // Coroutine function wrapper
        // Responsible for calling the function and dealing with panicking
        let wrapper = move || -> ! {
            let ret = unsafe {
                let puller_ref = puller_ref.clone();
                try(|| {
                    let coro_ref: &mut SymmetricCoroutine<T> = &mut *puller_ref.coro;
                    coro_ref.state = State::Running;
                    f(puller_ref)
                })
            };

            unsafe {
                let coro_ref: &mut SymmetricCoroutine<T> = &mut *puller_ref.coro;
                coro_ref.state = State::Finished;
            }

            let is_panicked = match ret {
                Ok(..) => false,
                Err(err) => {
                    if let None = err.downcast_ref::<ForceUnwind>() {
                        {
                            let msg = match err.downcast_ref::<&'static str>() {
                                Some(s) => *s,
                                None => {
                                    match err.downcast_ref::<String>() {
                                        Some(s) => &s[..],
                                        None => "Box<Any>",
                                    }
                                }
                            };

                            let name = coro_ref.name().unwrap_or("<unnamed>");
                            error!("Coroutine '{}' panicked at '{}'", name, msg);
                        }

                        coro_ref.result = Some(Err(Error::Panicking(err)));
                        true
                    } else {
                        false
                    }
                }
            };

            loop {
                if is_panicked {
                    coro_ref.result = Some(Err(Error::Panicked));
                }

                unsafe {
                    // Yield back to parent, if the parent exists
                    coro_ref.yield_back();
                }
            }
        };

        coro.context.init_with(coroutine_initialize, 0, Box::new(wrapper), &mut stack);
        coro.stack = Some(stack);
        coro
    }

    unsafe fn yield_to(&mut self,
                       target: &mut SymmetricCoroutine<T>,
                       mut data: Option<T>)
                       -> Result<Option<T>> {
        self.result = Some(Ok(&mut data));
        target.parent.set(self);
        Context::swap(&mut self.context, &target.context);

        if let State::ForceUnwind = self.state {
            begin_unwind(ForceUnwind, &(file!(), line!()));
        }

        match self.result.take() {
            None => Ok(None),
            Some(Ok(x)) => Ok((*x).take()),
            Some(Err(err)) => Err(err),
        }
    }

    unsafe fn yield_back(&mut self) -> Option<T> {
        let par = self.parent.clone();
        self.yield_to(&mut *par.get(), None).unwrap()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| &s[..])
    }

    fn take_data(&mut self) -> Option<T> {
        match self.result.take() {
            None => None,
            Some(Ok(x)) => unsafe { (*x).take() },
            _ => unreachable!("Coroutine is panicking"),
        }
    }

    unsafe fn force_unwind(&mut self) {
        if let State::Running = self.state {
            self.state = State::ForceUnwind;
            let mut tmp = SymmetricCoroutine::empty();

            let _ = try(|| {
                tmp.yield_to(self, None).unwrap();
            });
        }
    }
}

impl<T> Drop for SymmetricCoroutine<T>
    where T: Send
{
    fn drop(&mut self) {
        unsafe {
            self.force_unwind();
        }
        STACK_POOL.with(|pool| unsafe {
            if let Some(stack) = self.stack.take() {
                (&mut *pool.get()).give_stack(stack);
            }
        });
    }
}

pub struct SymmetricCoroutineRef<T>
    where T: Send
{
    coro: *mut SymmetricCoroutine<T>,
}

impl<T> Copy for SymmetricCoroutineRef<T> where T: Send {}

impl<T> Clone for SymmetricCoroutineRef<T>
    where T: Send
{
    fn clone(&self) -> SymmetricCoroutineRef<T> {
        SymmetricCoroutineRef { coro: self.coro }
    }
}

unsafe impl<T> Send for SymmetricCoroutineRef<T> where T: Send {}

impl<T> SymmetricCoroutineRef<T>
    where T: Send
{
    pub unsafe fn yield_to(&self,
                           target: &mut SymmetricCoroutine<T>,
                           data: Option<T>)
                           -> Result<Option<T>> {
        (&mut *self.coro).yield_to(target, data)
    }

    pub unsafe fn yield_with(&self, data: Option<T>) -> Result<Option<T>> {
        let target: &mut SymmetricCoroutine<T> = &mut *self.parent;
        self.yield_to(target, None)
    }

    pub unsafe fn resume_with(&self, data: Option<T>) -> Result<Option<T>> {}

    #[inline]
    pub fn name(&self) -> Option<&str> {
        unsafe { (&*self.coro).name() }
    }

    #[inline]
    pub fn take_data(&self) -> Option<T> {
        unsafe {
            let coro: &mut SymmetricCoroutine<T> = transmute(self.coro);
            coro.take_data()
        }
    }
}
