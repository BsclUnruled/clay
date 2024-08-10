
use crate::clay::{
    var::{ToVar, Virtual},
    vm::{keys, runtime::Vm, signal::{Abort, Signal}}
    ,
};

use super::{args::Args, func::Func, Function};

pub struct Method {
    name: String,
    uukey:String
}

// #[derive(Debug)]
// pub struct Method {
//     name: Option<String>,
//     map: RefCell<HashMap<(TypeId,usize), Var>>,
//     basic: Var,
// }

impl Virtual for Method {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn call(&self, all: Args) -> Signal {
        let args = all.args();

        let obj = match args.get(0) {
            Some(v) => v.clone().unbox()?,
            None => return Err(Abort::ThrowString(format!("方法调用时缺少对象参数"))),
        };

        let method = obj.get(*all.vm(), &self.uukey())?.unbox()?;

        method.call(all)
    }
}

impl Method{
    pub fn new(name: Option<String>) -> Self {
        let uukey = format!("--{}--",uuid::Uuid::new_v4());
        Self{
            name:match name{
                Some(n) => n,
                None => format!("method@{}",uukey)
            },
            uukey
        }
    }

    pub fn uukey(&self) -> &str {&self.uukey}
    pub fn name(&self) -> &str {&self.name}
}

// impl Method {
//     pub fn new(name: Option<String>, basic: Var) -> Self {
//         Self {
//             name,
//             map: HashMap::new().into(),
//             basic,
//         }
//     }

//     pub fn with_map(name: Option<String>, map: HashMap<(TypeId,usize), Var>, basic: Var) -> Self {
//         Self {
//             name,
//             map: map.into(),
//             basic,
//         }
//     }

//     pub fn append(&self, class: &Var, method: Var) -> Signal {
//         let class = class.unbox()?;

//         match self
//             .map
//             .borrow_mut()
//             .insert(class.class_id(), method)
//         {
//             Some(v) => Ok(v),
//             None => Err(Abort::ThrowString(format!(
//                 "无法为{}插入方法",
//                 match &self.name {
//                     Some(n) => n.to_owned(),
//                     None => format!("method@{:p}", self),
//                 }
//             ))),
//         }
//     }
// }

pub fn global_init(vm: Vm) {
    let hc:(Function,String) = (
        r#impl,
        String::from("impl")
    );
    let hc = Func::from(hc);
    let _ = vm.get_context().def(
        vm,
        "impl",
        &hc.to_var(vm),
    );
}

fn r#impl(all: Args) -> Signal {
    //impl str MyType@(this)=_
    let args = all.args();

    let hc = match args.get(0) {
        Some(v) => v.clone().unbox()?,
        None => return Err(Abort::ThrowString(format!("方法实现时缺少方法参数"))),
    };

    let method:&Method = hc.cast()?;

    let class = match args.get(1) {
        Some(v) => v.clone().unbox()?,
        None => return Err(Abort::ThrowString(format!("方法实现时缺少类型参数"))),
    };

    let func = match args.get(2) {
        Some(v) => v,
        None => return Err(Abort::ThrowString(format!("方法实现时缺少函数参数"))),
    };

    let vm = *all.vm();

    class.get(vm,keys::INSTANCE_META)?
       .unbox()?
       .set(vm, method.uukey(), func)
}

// pub fn into_kv<T:'static>(func:fn(Args)->Signal,vm: Vm)->((TypeId,usize),Var){
//     ((TypeId::of::<T>(),0),Func::from(func).to_var(vm))
// }
