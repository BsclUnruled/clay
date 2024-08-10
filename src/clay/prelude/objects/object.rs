use crate::clay::var::{Var,Virtual};
use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt::Display;

#[derive(Debug)]
pub struct Object {
    this:RefCell<HashMap<String,Var>>
}

impl Object {
    pub fn new(map:HashMap<String,Var>) -> Self {
        Object{
            this:RefCell::new(map)
        }
    }
}

impl Display for Object{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Clay Object@{:p}",self)
    }
}

impl Virtual for Object{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// impl Object {
//     pub fn new() -> Self {
//         let mut core = HashMap::new();

//         //core.insert(keys::CLASS.to_string(),ctor());
//         core.insert(keys::SUPER.to_string(),undef());

//         Object{
//             this:RefCell::new(core)
//         }
//     }
//     pub fn ctor(_:Args)->Signal{
//         Object::new().to_cross().into()
//     }
// }