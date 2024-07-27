use crate::clay::var::{func::Func, Var};

impl From<Var> for Signal {
    fn from(value: Var) -> Self {
        Ok(value)
    }
}

pub type Signal = Result<Var,Abort>;

pub type ErrSignal<Ok> = Result<Ok,Abort>;

#[derive(Debug)]
pub enum Abort {
    Throw(Var),
    Break(Var),
    ThrowError(Box<dyn std::error::Error>),
    ThrowString(String),
}

impl Abort{
    pub fn in_func(self,func:&Func)->Self{
        match self{
            Self::Break(_)=>Self::ThrowString(
                format!("Error: 试图直接在函数 {} 中使用 break 语句",func.name())
            ),
            _=>todo!()
        };
        todo!("in_func")
    }
}

unsafe impl Send for Abort {}
unsafe impl Sync for Abort {}