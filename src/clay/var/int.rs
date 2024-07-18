pub use num::bigint::BigInt;
pub use num::ToPrimitive;
use crate::clay::var::func;
use crate::clay::vm::{error, keys};
use super::{func::Args, undef::undef, Cross, Var};

pub type Int = BigInt;

impl Var for Int {
    fn get(&self, name: &str) -> super::Cross {
        match name {
            keys::CLASS => ctor(),
            keys::SUPER => undef(),
            _ => undef(),
        }
    }
    fn set(&self, name: &str, _: super::Cross) {
        error::set_unsetable("Int", name)
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
