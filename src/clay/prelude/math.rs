use crate::clay::var::Var;
use crate::clay::vm::env::Env;
use crate::clay::vm::promise::Promise;
use crate::clay:: vm::signal::Abort;

pub fn add(env:&Env,args:&[Var])->Promise{
    let x_v = match args.get(0){
        Some(x) => x,
        None => return Err(Abort::ThrowString(
            format!("add: missing argument 0")
        )).into()
    };

    let y_v = match args.get(1){
        Some(y) => y,
        None => return Err(Abort::ThrowString(
            format!("add: missing argument 1")
        )).into()
    };

    let add = x_v.get(env,"#add");
    add.call(env, &[y_v.clone()])
}