use crate::clay::var::Cross;

impl From<Cross> for Signal {
    fn from(value: Cross) -> Self {
        Ok(value)
    }
}

pub type Signal = Result<Cross,Abort>;

#[derive(Debug)]
pub enum Abort {
    Throw(Cross),
    Break(Cross),
    ThrowError(Box<dyn std::error::Error>),
    ThrowString(String),
}