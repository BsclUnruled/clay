use crate::clay::vm::{env, runtime::Vm, CtxType, Token};

#[derive(Clone)]
pub struct Args<'l> {
    vm:Vm,
    args:&'l[Token],
    ctx:CtxType,
}

impl<'l> Args<'l> {
    pub fn new(vm:Vm, args:&'l[Token], ctx:CtxType) -> Self {
        Self{
            vm,
            args,
            ctx,
        }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }

    pub fn args(&self) -> &'l [Token] {
        self.args
    }

    pub fn ctx(&self) -> CtxType {
        self.ctx.clone()
    }
}

impl<'l> From<(Vm,&'l [Token])> for Args<'l> {
    fn from(
        (vm,args):(Vm,&'l [Token])
    ) -> Self {
        Self{
            vm,
            args,
            ctx:env::void_ctx(vm),
        }
    }
}

impl<'l> From<(Vm,&'l [Token],CtxType)> for Args<'l> {
    fn from(
        (vm,args,ctx)
            :(Vm,&'l [Token],CtxType)
    ) -> Self {
        Self{
            vm,
            args,
            ctx,
        }
    }
}