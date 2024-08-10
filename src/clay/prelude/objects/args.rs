use crate::clay::{var::Var, vm::{runtime::Vm, CtxType}};

#[derive(Clone)]
pub struct Args<'l> {
    vm:Vm,
    args:&'l[Var],
    ctx:Option<CtxType>,
}

impl<'l> Args<'l> {
    pub fn new(vm:Vm, args:&'l[Var]) -> Self {
        Self{
            vm,
            args,
            ctx:None,
        }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }

    pub fn args(&self) -> &'l [Var] {
        self.args
    }

    pub fn ctx(&self) -> Option<CtxType> {
        self.ctx.clone()
    }
}

impl<'l> From<(Vm,&'l [Var])> for Args<'l> {
    fn from(
        (vm,args):(Vm,&'l [Var])
    ) -> Self {
        Self{
            vm,
            args,
            ctx:None,
        }
    }
}

impl<'l> From<(Vm,&'l [Var],CtxType)> for Args<'l> {
    fn from(
        (vm,args,ctx)
            :(Vm,&'l [Var],CtxType)
    ) -> Self {
        Self{
            vm,
            args,
            ctx:Some(ctx),
        }
    }
}