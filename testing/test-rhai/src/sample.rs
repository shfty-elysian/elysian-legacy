use rhai::{Dynamic, Engine, EvalAltResult, NativeCallContext};

use crate::{Context, Evaluate};

/// Given a list of commands and a position,
/// run the commands with a context initialized using that position.
pub trait Sample: Evaluate + Send + Sync + std::fmt::Debug {
    fn sample(
        context: NativeCallContext,
        this: Self,
        position: Dynamic,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut ctx = Context::default();
        ctx.set("position", position);
        context.call_fn("evaluate", (this, ctx))
    }

    fn register_sample(engine: &mut Engine) {
        engine.register_fn("sample", Self::sample);
    }
}

impl<T> Sample for T where T: 'static + Clone + Evaluate + Send + Sync + std::fmt::Debug {}
