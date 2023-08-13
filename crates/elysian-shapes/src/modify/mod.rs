pub mod post_modifier;
pub mod pre_modifier;

use elysian_proc_macros::elysian_stmt;
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
        AsModule, DomainsDyn, ErasedHash, FunctionDefinition, FunctionIdentifier, InputDefinition,
        Module, SpecializationData, CONTEXT,
    },
};

use crate::shape::{DynShape, IntoShape, Shape};

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait PreModifier: Debug + AsModule + ErasedHash + DomainsDyn {}

pub trait IntoPreModifier: 'static + Sized + PreModifier {
    fn pre_modifier(self) -> Box<dyn PreModifier> {
        Box::new(self)
    }
}

impl<T> IntoPreModifier for T where T: 'static + PreModifier {}

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait PostModifier: Debug + AsModule + ErasedHash + DomainsDyn {}

pub trait IntoPostModifier: 'static + Sized + PostModifier {
    fn post_modifier(self) -> Box<dyn PostModifier> {
        Box::new(self)
    }
}

impl<T> IntoPostModifier for T where T: 'static + PostModifier {}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            state.write_u64(modifier.erased_hash());
        }
        state.write_u64(self.field.erased_hash());
        for modifier in &self.post_modifiers {
            state.write_u64(modifier.erased_hash());
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

impl AsModule for Modify {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let pre_modules: Vec<_> = self
            .pre_modifiers
            .iter()
            .map(|m| m.module(&spec.filter(m.domains_dyn())))
            .collect();

        let field_module = self.field.module(&spec.filter(self.field.domains_dyn()));

        let post_modules: Vec<_> = self
            .post_modifiers
            .iter()
            .map(|m| m.module(&spec.filter(m.domains_dyn())))
            .collect();

        let modify_module = Module::new(
            self,
            spec,
            FunctionDefinition {
                id: FunctionIdentifier::new_dynamic("modify".into()),
                public: true,
                inputs: vec![InputDefinition {
                    id: PropertyIdentifier(CONTEXT),
                    mutable: false,
                }],
                output: PropertyIdentifier(CONTEXT),
                block: post_modules
                    .iter()
                    .fold(
                        field_module.call(
                            pre_modules
                                .iter()
                                .fold(elysian_stmt! {CONTEXT}, |acc, next| next.call(acc)),
                        ),
                        |acc, next| next.call(acc),
                    )
                    .output()
                    .block(),
            },
        );

        let module = pre_modules
            .iter()
            .cloned()
            .fold(Module::default(), |acc, next| acc.concat(next))
            .concat(field_module)
            .concat(
                post_modules
                    .iter()
                    .cloned()
                    .fold(Module::default(), |acc, next| acc.concat(next)),
            )
            .concat(modify_module);

        module
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Modify {}

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
