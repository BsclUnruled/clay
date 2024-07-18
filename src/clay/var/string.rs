use super::{func::Args, to_cross, undef::undef, Cross, Var};
use crate::clay::var::func;
use crate::clay::vm::{error, keys};

impl Var for String {
    fn get(&self, name: &str) -> super::Cross {
        match name {
            keys::CLASS => ctor(),
            _ => undef(),
        }
    }

    fn set(&self, name: &str, _: super::Cross) {
        error::set_unsetable("Str", name)
    }
}

pub fn escape(s: &str) -> String{
    todo!()
}
pub fn template(s: &str) -> String{
    todo!()
}

thread_local! {
    static CTOR: Cross = func::new_ctor(
        Box::leak(Box::new(|_:Args|{
            to_cross(Box::new("".to_string()))
        }))
    );
}

pub fn ctor() -> Cross {
    CTOR.with(|f| f.clone())
}
