use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::module::StructDefinition;
use elysian_proc_macros::elysian_stmt;

use elysian_core::expr::IntoExpr;
use elysian_ir::{
    ast::IntoBlock,
    module::{
        AsIR, DomainsDyn, DynAsIR, FunctionDefinition, FunctionIdentifier, HashIR, InputDefinition,
        IntoAsIR, IntoRead, SpecializationData, CONTEXT,
    },
};

pub struct Select {
    default: DynAsIR,
    cases: Vec<(elysian_core::expr::Expr, DynAsIR)>,
}

impl Select {
    pub fn new(default: impl IntoAsIR) -> Self {
        Select {
            default: default.as_ir(),
            cases: Default::default(),
        }
    }

    pub fn case(mut self, cond: impl IntoExpr, then: impl IntoAsIR) -> Self {
        self.cases.push((cond.expr(), then.as_ir()));
        self
    }
}

impl Debug for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Select")
            .field("shapes", &self.cases)
            .finish()
    }
}

impl Hash for Select {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.default.hash_ir());
        for (_, shape) in &self.cases {
            state.write_u64(shape.hash_ir())
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

impl AsIR for Select {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("select".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prepared_shapes: Vec<(_, _)> = self
            .cases
            .iter()
            .map(|(k, v)| {
                let (a, b, c) = v.prepare(spec);
                (k, (v, a, b, c))
            })
            .collect();

        let (_, default_call, default_functions) =
            self.default.call(spec, elysian_stmt! { CONTEXT });

        let block = prepared_shapes
            .iter()
            .rev()
            .fold(default_call.output(), |acc, (k, (v, _, entry, _))| {
                elysian_ir::ast::Stmt::If {
                    cond: (*k).clone().into(),
                    then: entry
                        .call(v.arguments(PropertyIdentifier(CONTEXT).read()))
                        .output()
                        .box_stmt(),
                    otherwise: Some(acc.box_stmt()),
                }
            })
            .block();

        prepared_shapes
            .into_iter()
            .flat_map(|(_, (_, _, _, functions))| functions)
            .chain(default_functions)
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: false,
                }],
                output: CONTEXT.into(),
                block,
            })
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        self.cases
            .iter()
            .flat_map(|(_, v)| v.structs())
            .chain(self.default.structs())
            .collect()
    }
}
