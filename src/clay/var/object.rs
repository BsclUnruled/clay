use super::ToVar;
use super::Var;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Object {
    this:RefCell<HashMap<String,Var>>
}

impl ToVar for Object{}

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