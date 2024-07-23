use asymmetric;
use options::Options;

/// Coroutine configuration. Provides detailed control over the properties and behavior of new Coroutines.
///
/// ```rust
/// use coroutine::AsymmetricBuilder;
///
/// let coro = AsymmetricBuilder::new().name(format!("Coroutine #{}", 1))
///                                    .stack_size(4096)
///                                    .spawn(|_, _| println!("Hello world!!"));
///
/// coro.resume().unwrap();
/// ```
pub struct AsymmetricBuilder {
    opts: Options,
}

impl AsymmetricBuilder {
    /// Generate the base configuration for spawning a Coroutine, from which configuration methods can be chained.
    pub fn new() -> AsymmetricBuilder {
        AsymmetricBuilder { opts: Default::default() }
    }

    /// Name the Coroutine-to-be. Currently the name is used for identification only in panic messages.
    pub fn name(mut self, name: String) -> AsymmetricBuilder {
        self.opts.name = Some(name);
        self
    }

    /// Set the size of the stack for the new Coroutine.
    pub fn stack_size(mut self, size: usize) -> AsymmetricBuilder {
        self.opts.stack_size = size;
        self
    }

    /// Spawn a new Coroutine, and return a handle for it.
    pub fn spawn<F>(self, f: F) -> asymmetric::Handle
        where F: FnOnce(&mut asymmetric::Coroutine, usize) -> usize + 'static
    {
        asymmetric::Coroutine::spawn_opts(f, self.opts)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asymmetric_builder_basic() {
        let ret = AsymmetricBuilder::new()
            .name("Test builder".to_string())
            .spawn(move |me, _| me.yield_with(1))
            .resume(1);
        assert_eq!(1, ret.unwrap());
    }
}
