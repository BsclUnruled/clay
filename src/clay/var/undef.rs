use crate::clay::vm;

use super::Var;

struct Undef();

impl Var for Undef {
    fn get(&self, name:&str)->super::Cross {
        match name {
            "?"=>undef(),
            _=>vm::error::throw(&format!("Error:读取undef的属性{}",name))
        }
    }
    fn set(&self, name:&str, value:super::Cross) {
        vm::error::throw(&format!("Error:尝试设置undef的属性{}",name))
    }
}

thread_local! {
    static UD:super::Cross = super::to_cross(Undef());
}

pub fn undef() -> super::Cross {
    UD.with(|ud| ud.clone())
}