pub struct Runtime {
    async_runtime:tokio::runtime::Runtime,
}

impl Runtime {
    pub fn async_runtime(&self) -> &tokio::runtime::Runtime {
        &self.async_runtime
    }
    pub fn new() -> Self {
        Self {
            async_runtime:tokio::runtime::Runtime::new().unwrap(),
        }
    }
}