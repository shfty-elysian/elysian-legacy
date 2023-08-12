pub mod post_modifier;
pub mod pre_modifier;

pub use post_modifier::*;
pub use pre_modifier::*;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::IntoBlock,
    module::{
        AsIR, DomainsDyn, FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead,
        Prepare, SpecializationData, StructDefinition, CONTEXT,
    },
};

use crate::shape::{DynShape, IntoShape, Shape};

pub trait PreModifier: AsIR {}

pub trait IntoPreModifier: 'static + Sized + PreModifier {
    fn pre_modifier(self) -> Box<dyn PreModifier> {
        Box::new(self)
    }
}

impl<T> IntoPreModifier for T where T: 'static + PreModifier {}

pub trait PostModifier: AsIR {}

pub trait IntoPostModifier: 'static + Sized + PostModifier {
    fn post_modifier(self) -> Box<dyn PostModifier> {
        Box::new(self)
    }
}

impl<T> IntoPostModifier for T where T: 'static + PostModifier {}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Modify {
    pre_modifiers: Vec<Box<dyn PreModifier>>,
    field: DynShape,
    post_modifiers: Vec<Box<dyn PostModifier>>,
}

impl Modify {
    pub fn new(field: impl IntoShape) -> Self {
        Modify {
            pre_modifiers: Default::default(),
            field: field.shape(),
            post_modifiers: Default::default(),
        }
    }

    pub fn push_pre(mut self, modifier: impl IntoPreModifier) -> Self {
        self.pre_modifiers.push(modifier.pre_modifier());
        self
    }

    pub fn push_post(mut self, post_modifier: impl IntoPostModifier) -> Self {
        self.post_modifiers.push(post_modifier.post_modifier());
        self
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

pub trait IntoModify: 'static + Sized + Shape {
    fn modify(self) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T> IntoModify for T where T: 'static + Sized + Shape {}
