use std::{error::Error, fmt::{Debug, Display}};

use crate::clay::var::{ToVar, Virtual};

use super::{runtime::Vm, Abort};

pub fn throw(vm:&Vm, message: &str) -> Abort {
    Abort::Throw(VmError::new(message, None).to_var(*vm))
}

pub struct VmError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl Virtual for VmError {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
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

impl Debug for VmError {
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

// impl Add<Box<dyn Error>> for VmError {
//     type Output = Box<dyn Error>;

//     fn add(self, rhs: Box<dyn Error>) -> Self::Output {
//         Box::new(VmError {
//             message: self.message,
//             source: Some(rhs),
//         })
//     }
// }

impl From<&str> for VmError {
    fn from(message: &str) -> Self {
        Self::new(message, None)
    }
}

// impl From<VmError> for Abort {
//     fn from(e: VmError) -> Self {
//         Abort::Throw(Box::new(e))
//     }
// }

pub fn set_unsetable(vm:&Vm,type_name: &str, property_name: &str)->Abort{
    throw(vm,&format!(
        "set unsetable\n\t\t尝试设置{}的属性{}",
        property_name, type_name
    ))
}

pub fn get_ungetable(vm:&Vm,type_name: &str, property_name: &str)->Abort{
    throw(vm,&format!(
        "get ungetable\n\t\t尝试获取{}的属性{}",
        property_name, type_name
    ))
}

pub fn set_undef(vm:&Vm,property_name: &str)->Abort{
    set_unsetable(vm,"undef", property_name)
}

pub fn use_dropped(vm:&Vm)->Abort{
    throw(vm,&format!("use dropped\n\t\t变量已回收"))
}

pub fn async_scheduler_error(vm:&Vm)->Abort{
    throw(vm,"schedule failed\n\t\tclay异步函数调度失败")
}

pub fn not_a_func(vm:&Vm)->Abort{
    throw(vm,&format!("not a function\n\t\t不是函数"))
}

pub fn def_undefable(vm:&Vm,type_name: &str, property_name: &str)->Abort{
    throw(vm,&format!(
        "def undefable\n\t\t尝试在{}上定义『{}』属性",
        property_name, type_name
    ))
}

pub fn cast_error(expcet: &str, actual:&str)->Abort{
    Abort::ThrowString(format!(
r"cast error
                类型转换失败
                期望类型: {}, 实际类型: {}", expcet, actual))
}
