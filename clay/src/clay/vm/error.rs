use std::{error::Error, fmt::Display, ops::Add};

use super::Abort;

pub fn throw(message: &str) -> Abort {
    Abort::ThrowError(VmError::new(message, None).into())
}

#[derive(Debug)]
pub struct VmError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.source {
            None => format!("{}", self.message),
            Some(ref e) => format!("{}\n\tfrom: {}", self.message, e),
        };
        write!(f, "Error: {}", message)
    }
}

impl Error for VmError {}

impl VmError {
    pub fn new(message: &str, source: Option<Box<dyn Error>>) -> Self {
        Self {
            message: message.to_string(),
            source,
        }
    }
}

impl Add<Box<dyn Error>> for VmError {
    type Output = Box<dyn Error>;

    fn add(self, rhs: Box<dyn Error>) -> Self::Output {
        Box::new(VmError {
            message: self.message,
            source: Some(rhs),
        })
    }
}

impl From<&str> for VmError {
    fn from(message: &str) -> Self {
        Self::new(message, None)
    }
}

impl From<VmError> for Abort {
    fn from(e: VmError) -> Self {
        Abort::ThrowError(Box::new(e))
    }
}

pub fn set_unsetable(type_name: &str, property_name: &str)->Abort{
    throw(&format!(
        "set unsetable\n\t\t尝试设置{}的属性{}",
        property_name, type_name
    ))
}

pub fn get_ungetable(type_name: &str, property_name: &str)->Abort{
    throw(&format!(
        "get ungetable\n\t\t尝试获取{}的属性{}",
        property_name, type_name
    ))
}

pub fn set_undef(property_name: &str)->Abort{
    set_unsetable("undef", property_name)
}

pub fn use_dropped()->Abort{
    throw(&format!("use dropped\n\t\t变量已回收"))
}

pub fn async_scheduler_error()->Abort{
    Abort::ThrowError(
        Box::new(VmError::new(
            "schedule failed\n\t\tclay异步函数调度失败",
            None,
        ))
    )
}

pub fn not_a_func()->Abort{
    throw(&format!("not a function\n\t\t不是函数"))
}

pub fn def_undefable(type_name: &str, property_name: &str)->Abort{
    throw(&format!(
        "def undefable\n\t\t尝试在{}上定义『{}』属性",
        property_name, type_name
    ))
}
