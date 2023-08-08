use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    ast::IntoBlock,
    module::{
        AsIR, DomainsDyn, DynAsIR, FunctionDefinition, FunctionIdentifier, InputDefinition,
        IntoAsIR, IntoRead, PropertyIdentifier, SpecializationData, StructDefinition, CONTEXT,
    },
};

pub struct Modify {
    pre_modifiers: Vec<DynAsIR>,
    field: DynAsIR,
    post_modifiers: Vec<DynAsIR>,
}

impl Modify {
    pub fn new(field: impl IntoAsIR) -> Self {
        Modify {
            pre_modifiers: Default::default(),
            field: field.as_ir(),
            post_modifiers: Default::default(),
        }
    }

    pub fn push_pre(mut self, pre_modifier: impl IntoAsIR) -> Self {
        self.pre_modifiers.push(pre_modifier.as_ir());
        self
    }

    pub fn push_post(mut self, post_modifier: impl IntoAsIR) -> Self {
        self.post_modifiers.push(post_modifier.as_ir());
        self
    }
}

impl Debug for Modify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Modify")
            .field("pre_modifiers", &self.pre_modifiers)
            .field("field", &self.field)
            .field("post_modifiers", &self.post_modifiers)
            .finish()
    }
}

impl Hash for Modify {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for modifier in &self.pre_modifiers {
            state.write_u64(modifier.hash_ir());
        }
        state.write_u64(self.field.hash_ir());
        for modifier in &self.post_modifiers {
            state.write_u64(modifier.hash_ir());
        }
    }
}

impl DomainsDyn for Modify {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.pre_modifiers
            .iter()
            .flat_map(|t| t.domains_dyn())
            .chain(self.field.domains_dyn())
            .chain(self.post_modifiers.iter().flat_map(|t| t.domains_dyn()))
            .collect()
    }
}

impl AsIR for Modify {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("modify".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let pre_modifiers: Vec<_> = self
            .pre_modifiers
            .iter()
            .map(|t| {
                let (_, b, c) = t.prepare(spec);
                (t, b, c)
            })
            .collect();

        let (_, field_entry_point, field_functions) = self.field.prepare(spec);

        let post_modifiers: Vec<_> = self
            .post_modifiers
            .iter()
            .map(|t| {
                let (_, b, c) = t.prepare(spec);
                (t, b, c)
            })
            .collect();

        let pre_functions: Vec<_> = pre_modifiers
            .iter()
            .flat_map(|(_, _, c)| c)
            .cloned()
            .collect();

        let post_functions: Vec<_> = post_modifiers
            .iter()
            .flat_map(|(_, _, c)| c)
            .cloned()
            .collect();

        pre_functions
            .into_iter()
            .chain(field_functions)
            .chain(post_functions)
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: PropertyIdentifier(CONTEXT),
                    mutable: false,
                }],
                output: PropertyIdentifier(CONTEXT),
                block: post_modifiers
                    .iter()
                    .fold(
                        field_entry_point.call(
                            self.field.arguments(
                                pre_modifiers.iter().fold(
                                    PropertyIdentifier(CONTEXT).read(),
                                    |acc, (t, entry, _)| entry.call(t.arguments(acc)),
                                ),
                            ),
                        ),
                        |acc, (t, entry, _)| entry.call(t.arguments(acc)),
                    )
                    .output()
                    .block(),
            })
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        self.pre_modifiers
            .iter()
            .flat_map(|t| t.structs())
            .chain(self.field.structs())
            .chain(self.post_modifiers.iter().flat_map(|t| t.structs()))
            .collect()
    }
}

pub trait IntoModify: 'static + Sized + AsIR {
    fn modify(self) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T> IntoModify for T where T: 'static + Sized + AsIR {}
