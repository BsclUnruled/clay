// use std::cell::RefCell;
// use std::collections::HashMap;
// use crate::clay::var::Cross;
// use std::cell::Cell;

// pub mod code_runner;
// pub use self::code_runner::CodeRunner;

// pub struct Coro{
//     closure:RefCell<HashMap<String,Cross>>,
//     coro_iter:CodeRunner,
//     stage:Cell<Stage>
// }

// #[derive(Debug,Clone,Copy)]
// pub enum Stage{
//     Init,
//     Waiting,
//     Running,
//     Finished
// }