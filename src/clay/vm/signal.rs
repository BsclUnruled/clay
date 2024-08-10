use std::fmt::Display;
use crate::clay::{prelude::objects::func::Func, var::Var};

impl From<Var> for Signal {
    fn from(value: Var) -> Self {
        Ok(value)
    }
}

pub type Signal = Result<Var, Abort>;

pub type ErrSignal<Ok> = Result<Ok, Abort>;

#[derive(Debug, Clone)]
pub enum Abort {
    Throw(Var),
    Break(Var),
    ThrowString(String),
    Exit,

    Return(Var),
    Continue,
    End(Var),
    Nop
}

impl Display for Abort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Abort::Throw(_) => write!(f, "todo"),
            Abort::Break(_) => write!(f, "[Break]"),
            Abort::ThrowString(s) => write!(f, "{}", s),
            Abort::Exit => write!(f, "[Exit]"),
            Abort::Return(_) => write!(f, "[Return]"),
            Abort::Continue => write!(f, "[Continue]"),
            Abort::End(_) => write!(f, "[End]"),
            Abort::Nop => write!(f, "[Nop]"),
        }
    }
}

impl Abort {
    pub fn as_string(&self) -> String {
        match self {
            Abort::Throw(_) => "Throe".into(),
            Abort::Break(_) => "Break".into(),
            Abort::Continue => "Continue".into(),
            Abort::Return(_) => "Return".into(),
            Abort::End(_) => "End".into(),
            Abort::Exit => "Exit".into(),
            Abort::ThrowString(s) => s.to_owned(),
            Abort::Nop => "Nop".into(),
        }
    }
    pub fn in_func(self, func: &Func) -> Self {
        match self {
            Self::Break(_) => Self::ThrowString(format!(
                "Error: 试图直接在函数 {} 中使用 break 语句",
                func.name()
            )),
            _ => todo!(),
        };
        todo!("in_func")
    }
}

unsafe impl Send for Abort {}
unsafe impl Sync for Abort {}
