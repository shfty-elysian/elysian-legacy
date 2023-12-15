use rhai::{Dynamic, EvalAltResult, ImmutableString, Map, NativeCallContext};

use crate::{Context, Evaluate};

/// A set of atomic Context -> Context computations that can be filtered.
///
/// Each channel is executed concurrently, and receives a copy of the input context.
/// The results are aggregated, and merged into the input context to produce a final result.
/// Multiple channels attempting to modify the same data will cause an error.
#[derive(Debug, Default, Clone, Hash)]
pub struct Channels {
    channels: Map,
    filter_key: ImmutableString,
}

impl Channels {
    pub fn new(channels: Map) -> Self {
        Channels {
            channels,
            filter_key: "channels".into(),
        }
    }

    pub fn new_filter(channels: Map, filter_key: ImmutableString) -> Self {
        Channels {
            channels,
            filter_key,
        }
    }
}

impl Evaluate for Channels {
    /// Given a map of property names to function pointers,
    /// call them with parameters read from the provided context,
    /// and return the context updated with their output.
    fn evaluate(
        context: NativeCallContext,
        this: Self,
        mut ctx: Context,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        // Create empty map to hold new data
        let mut out = Map::default();

        let filter = ctx
            .get(this.filter_key.as_str())
            .ok()
            .map(|filter| {
                filter
                    .clone()
                    .into_array()?
                    .into_iter()
                    .map(|item| item.into_string())
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_else(|| this.channels.keys().map(|key| key.to_string()).collect());

        let channels = this.channels.into_iter().filter_map(|(key, value)| {
            if filter.iter().any(|cand| cand.as_str() == key.as_str()) {
                Some(value)
            } else {
                None
            }
        });

        // Aggregate results of active command domain functions
        for domain_fn in channels {
            // Evaluate domain function
            let mut res = context.call_fn::<Map>("evaluate", (domain_fn, ctx.clone()))?;

            // Throw an error if the output already contains any of the resulting properties
            if let Some(key) = res.keys().find(|key| out.contains_key(*key)) {
                return Err(Box::new(EvalAltResult::ErrorRuntime(
                    format!("Non-atomic channel: Conflict in {} output", key).into(),
                    context.position(),
                )));
            }

            out.append(&mut res);
        }

        // Merge results into context
        ctx.append(out);

        Ok(Dynamic::from(ctx))
    }
}
