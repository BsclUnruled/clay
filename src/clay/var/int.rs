use num::bigint::BigInt;
pub use num::ToPrimitive;

use crate::clay::vm::{error, keys};

use super::{func::Args, undef::undef, Cross, Var};

pub struct Int(BigInt);

impl Var for Int {
    fn get(&self, name: &str) -> super::Cross {
        match name {
            keys::CLASS => ctor(),
            keys::SUPER => undef(),
            _=>undef()
        }
    }
    fn set(&self, name: &str, _: super::Cross) {
        error::set_unsetable("Int", name)
    }
}

impl Int {
    pub fn new(value: i64) -> Self {
        Self(value.into())
    }
    
    fn ctor(_:Args)->Cross{
        super::to_cross(Box::new(Int(0.into())))
    }
}

thread_local! {
    static CTOR:Cross = super::func::new_ctor(&Int::ctor);
}

pub fn ctor() -> Cross {
    CTOR.with(|c| c.clone())
}