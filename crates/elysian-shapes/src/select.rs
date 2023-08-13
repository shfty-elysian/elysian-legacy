use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::ast::IntoBlock;
use elysian_ir::module::{AsModule, Module};
use elysian_proc_macros::elysian_stmt;

use elysian_core::expr::IntoExpr;
use elysian_ir::module::{
    DomainsDyn, ErasedHash, FunctionDefinition, FunctionIdentifier, InputDefinition,
    SpecializationData, CONTEXT,
};

use crate::shape::{DynShape, IntoShape, Shape};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Select {
    default: DynShape,
    cases: Vec<(elysian_core::expr::Expr, DynShape)>,
}

impl Select {
    pub fn new(default: impl IntoShape) -> Self {
        Select {
            default: default.shape(),
            cases: Default::default(),
        }
    }

    pub fn case(mut self, cond: impl IntoExpr, then: impl IntoShape) -> Self {
        self.cases.push((cond.expr(), then.shape()));
        self
    }
}

impl Hash for Select {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.default.erased_hash());
        for (_, shape) in &self.cases {
            state.write_u64(shape.erased_hash())
        }
    }
}

impl DomainsDyn for Select {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.cases
            .iter()
            .flat_map(|(_, t)| t.domains_dyn())
            .chain(self.default.domains_dyn())
            .collect()
    }
}

impl AsModule for Select {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let prepared_shapes: Vec<(_, _)> = self
            .cases
            .iter()
            .map(|(k, v)| {
                let module = v.module(spec);
                (k, module)
            })
            .collect();

        let default_module = self.default.module(spec);
        let default_call = default_module.call(elysian_stmt! { CONTEXT });

        let block = prepared_shapes
            .iter()
            .rev()
            .fold(default_call.output(), |acc, (k, v)| {
                elysian_ir::ast::Stmt::If {
                    cond: (*k).clone().into(),
                    then: v.call(elysian_stmt! {CONTEXT}).output().box_stmt(),
                    otherwise: Some(acc.box_stmt()),
                }
            })
            .block();

        prepared_shapes
            .into_iter()
            .fold(Module::default(), |acc, (_, next)| acc.concat(next))
            .concat(default_module)
            .concat(Module::new(
                self,
                spec,
                FunctionDefinition {
                    id: FunctionIdentifier::new_dynamic("select".into()),
                    public: false,
                    inputs: vec![InputDefinition {
                        id: CONTEXT.into(),
                        mutable: false,
                    }],
                    output: CONTEXT.into(),
                    block,
                },
            ))
    }
}

#[typetag::serde]
impl Shape for Select {}
