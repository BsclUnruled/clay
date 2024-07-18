pub use crate::clay::var::Var;
use crate::clay::var::Cross;
use crate::clay::vm::error;
use crate::clay::vm::keys;
use crate::clay::var::{func,int::*, func::Args};
use super::undef::undef;

impl Var for f64{
    fn get(&self, name: &str) -> Cross {
        match name {
            keys::CLASS=>ctor(),
            _=>undef()
        }
    }
    fn set(&self, name: &str, _: Cross) {
        error::set_unsetable("Float", name)
    }
}

thread_local! {
    static CTOR: Cross = func::new_ctor(
        Box::leak(Box::new(|_:Args|{
            super::to_cross(Box::<BigInt>::new(0.into()))
        }))
    );
}

pub fn ctor() -> Cross {
    CTOR.with(|c| c.clone())
}