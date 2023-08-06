use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ir::ast::IntoBlock;
use crate::ir::module::{
    AsIR, DynAsIR, FunctionDefinition, FunctionIdentifier, HashIR, InputDefinition,
    PropertyIdentifier, SpecializationData, CONTEXT,
};
use crate::ir::module::{DomainsDyn, IntoRead};

pub struct Select {
    pub cases: Vec<(elysian_core::ast::expr::Expr, DynAsIR)>,
    pub default: DynAsIR,
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
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("select".into()).specialize(spec)
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

        let (_, default_entry, default_functions) = self.default.prepare(spec);

        let block = prepared_shapes
            .iter()
            .rev()
            .fold(
                default_entry
                    .call(self.default.arguments(PropertyIdentifier(CONTEXT).read()))
                    .output(),
                |acc, (k, (v, _, entry, _))| crate::ir::ast::Stmt::If {
                    cond: (*k).clone().into(),
                    then: Box::new(
                        entry
                            .call(v.arguments(PropertyIdentifier(CONTEXT).read()))
                            .output(),
                    ),
                    otherwise: Some(Box::new(acc)),
                },
            )
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

    fn structs(&self) -> Vec<crate::ir::module::StructDefinition> {
        self.cases.iter().flat_map(|(_, v)| v.structs()).collect()
    }
}

pub trait IntoSelect<U> {
    fn select(self, default: U) -> Select;
}

impl<T, U> IntoSelect<U> for T
where
    T: IntoIterator<Item = (elysian_core::ast::expr::Expr, DynAsIR)>,
    U: 'static + AsIR,
{
    fn select(self, default: U) -> Select {
        let cases: Vec<(_, _)> = self.into_iter().collect();
        assert!(cases.len() >= 1, "Select must have at least one shape");
        Select {
            cases,
            default: Box::new(default),
        }
    }
}
