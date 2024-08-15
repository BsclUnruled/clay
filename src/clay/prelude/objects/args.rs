use crate::clay::{var::Var, vm::{env, runtime::Vm, CtxType}};

#[derive(Clone)]
pub struct Args<'l> {
    vm:Vm,
    args:&'l[Var],
    ctx:CtxType,
}

impl<'l> Args<'l> {
    pub fn new(vm:Vm, args:&'l[Var], ctx:CtxType) -> Self {
        Self{
            vm,
            args,
            ctx,
        }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }

    pub fn args(&self) -> &'l [Var] {
        self.args
    }

    pub fn ctx(&self) -> CtxType {
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
            ctx:env::void_ctx(vm),
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
            ctx,
        }
    }
}