mod image;
pub use self::image::*;

use rhai::{Dynamic, Engine, EvalAltResult, NativeCallContext};

pub trait Buffer: 'static + Clone + Send + Sync {
    fn map<'a>(
        context: NativeCallContext,
        this: &'a mut Self,
        shape: Dynamic,
    ) -> Result<(), Box<EvalAltResult>>;

    /// Register this type with the engine
    fn register(engine: &mut Engine) {
        engine.register_type::<Self>().register_fn("map", Self::map);
    }
}
