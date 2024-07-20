use crate::clay::var::Cross;

impl From<Cross> for Signal {
    fn from(value: Cross) -> Self {
        Ok(value)
    }
}

pub type Signal = Result<Cross,Abort>;

pub enum Abort {
    Throw(Cross),
    Break(Cross),
}