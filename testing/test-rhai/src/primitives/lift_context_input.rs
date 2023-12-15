use rhai::{Array, Dynamic, EvalAltResult, FnPtr, Identifier, Map, NativeCallContext};

use crate::{Context, Evaluate};

/// Lifts a function to read its parameters from a map by name
#[derive(Debug, Clone, Hash)]
pub struct LiftContextInput(FnPtr);

impl LiftContextInput {
    pub fn new(f: FnPtr) -> Self {
        LiftContextInput(f)
    }
}

impl Evaluate for LiftContextInput {
    /// Given a function pointer,
    /// read its parameters from the provided context,
    /// curry them in, and return an updated function pointer
    fn evaluate(
        context: NativeCallContext,
        mut this: Self,
        mut ctx: Context,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        let name = this.0.fn_name().to_string();

        // Fetch function metadata
        let meta_list: Array = context.call_fn("get_fn_metadata_list", (name.clone(),))?;

        // Throw an exception if the function does not exist,
        // or is multiply-defined
        if meta_list.len() == 0 {
            return Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("No function with name {}", &name).into(),
                context.position(),
            )));
        } else if meta_list.len() > 1 {
            return Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Function {} is multiply-defined", &name).into(),
                context.position(),
            )));
        }

        // Assume the first function, as no overloading is expected
        let meta: Map = meta_list[0].clone().cast();

        // Iterate over metadata parameters
        for param in meta["params"]
            .clone()
            .cast::<Array>()
            .into_iter()
            .map(|foo| Ok(Identifier::from(foo.into_immutable_string()?)))
            .collect::<Result<Vec<_>, Box<EvalAltResult>>>()?
        {
            // Skip params prefixed with an underscore
            if param.chars().next().unwrap() == '_' {
                continue;
            }

            // Throw an exception if a required param is missing
            if !ctx.contains(&param) {
                return Err(Box::new(EvalAltResult::ErrorRuntime(
                    format!(
                        "Function {} requires parameter {}, but it is missing",
                        name, param
                    )
                    .into(),
                    context.position(),
                )));
            }

            // Curry param into function
            this.0.add_curry(ctx.get(&param)?.clone());
        }

        this.0.call_within_context(&context, ())
    }
}
