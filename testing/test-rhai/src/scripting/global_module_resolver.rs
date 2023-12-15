use rhai::{Engine, EvalAltResult, Module, ModuleResolver, Position, Shared};

/// Wrapper to expose public imported functions to the global namespace
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalModuleResolver<T: ModuleResolver>(T);

impl<T: ModuleResolver> GlobalModuleResolver<T> {
    pub fn new(resolver: T) -> Self {
        GlobalModuleResolver(resolver)
    }
}

impl<T: ModuleResolver> ModuleResolver for GlobalModuleResolver<T> {
    fn resolve(
        &self,
        engine: &Engine,
        source: Option<&str>,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let mut module = (*self.0.resolve(engine, source, path, pos)?).clone();

        for (access, def) in module
            .iter_script_fn_info()
            .map(|(_, access, _, _, def)| (access, (**def).clone()))
            .collect::<Vec<_>>()
        {
            match access {
                rhai::FnAccess::Public => {
                    let hash = module.set_script_fn(def.clone());
                    module.update_fn_namespace(hash, rhai::FnNamespace::Global);
                }
                _ => (),
            }
        }

        Ok(Shared::new(module))
    }
}
