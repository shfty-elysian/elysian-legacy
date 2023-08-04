use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_proc_macros::elysian_stmt;

use crate::ir::ast::IntoBlock;
use crate::ir::module::{
    AsIR, DynAsIR, FunctionDefinition, FunctionIdentifier, HashIR, InputDefinition,
    PropertyIdentifier, SpecializationData, CONTEXT,
};
use crate::ir::module::{DomainsDyn, IntoRead};

pub struct Select {
    pub shapes: Vec<(elysian_core::ast::expr::Expr, DynAsIR)>,
}

impl Debug for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Select")
            .field("shapes", &self.shapes)
            .finish()
    }
}

impl Hash for Select {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (_, shape) in &self.shapes {
            state.write_u64(shape.hash_ir())
        }
    }
}

impl DomainsDyn for Select {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.shapes
            .iter()
            .flat_map(|(_, t)| t.domains_dyn())
            .collect()
    }
}

impl AsIR for Select {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("select")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prepared_shapes: Vec<(_, _)> = self
            .shapes
            .iter()
            .map(|(k, v)| {
                let (a, b, c) = v.prepare(spec);
                (k, (v, a, b, c))
            })
            .collect();

        let block = prepared_shapes
            .iter()
            .rev()
            .fold(
                elysian_stmt! { return CONTEXT },
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
        self.shapes.iter().flat_map(|(_, v)| v.structs()).collect()
    }
}

pub trait IntoSelect {
    fn select(self) -> Select;
}

impl<T> IntoSelect for T
where
    T: IntoIterator<Item = (elysian_core::ast::expr::Expr, DynAsIR)>,
{
    fn select(self) -> Select {
        let shapes: Vec<(_, _)> = self.into_iter().collect();
        assert!(shapes.len() >= 1, "Select must have at least one shape");
        Select { shapes }
    }
}
