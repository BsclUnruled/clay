use super::{runtime::Vm, CtxType};

#[derive(Clone)]
pub struct Env{
    vm:Vm,
    ctx:CtxType,
}

impl Env{
    pub fn new(vm:Vm, ctx:CtxType) -> Self {
        Self{
            vm,
            ctx,
        }
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }

    pub fn ctx(&self) -> CtxType {
        self.ctx.clone()
    }
}