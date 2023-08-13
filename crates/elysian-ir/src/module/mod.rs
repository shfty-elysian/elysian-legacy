mod domains;
mod erased_hash;
mod function_definition;
mod properties;
mod specialization_data;
mod struct_definition;
mod ty;

use std::{
    borrow::Cow,
    collections::hash_map::RandomState,
    hash::{Hash, Hasher},
};

pub use domains::*;
use elysian_core::{identifier::Identifier, property_identifier::PropertyIdentifier, uuid::Uuid};
pub use erased_hash::*;
pub use function_definition::*;
use indexmap::IndexSet;
pub use properties::*;
pub use specialization_data::*;
pub use struct_definition::*;
pub use ty::*;

use crate::{
    ast::{
        Block, Expr, Stmt, COMBINE_CONTEXT, MATRIX2_STRUCT, MATRIX3_STRUCT, MATRIX4_STRUCT,
        VECTOR2_STRUCT, VECTOR3_STRUCT, VECTOR4_STRUCT,
    },
    property,
};

pub const BUILTIN_STRUCTS: &'static [&'static StructDefinition] = &[
    VECTOR2_STRUCT,
    VECTOR3_STRUCT,
    VECTOR4_STRUCT,
    MATRIX2_STRUCT,
    MATRIX3_STRUCT,
    MATRIX4_STRUCT,
];

pub const CONTEXT: Identifier = Identifier::new("Context", 595454262490629935);
property!(
    CONTEXT,
    CONTEXT_PROP_DEF,
    Type::Struct(StructIdentifier(CONTEXT))
);

#[derive(Debug, Clone)]
pub struct Module {
    pub struct_definitions: Vec<StructDefinition>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub entry_point: FunctionIdentifier,
    pub arguments: Vec<Expr>,
    pub hash: u64,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            struct_definitions: Default::default(),
            function_definitions: Default::default(),
            entry_point: FunctionIdentifier(Identifier {
                name: Default::default(),
                uuid: Uuid::nil(),
            }),
            arguments: Default::default(),
            hash: Default::default(),
        }
    }
}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Module {
    pub fn new(
        shape: &(impl ErasedHash + DomainsDyn),
        spec: &SpecializationData,
        entry_function: FunctionDefinition,
    ) -> Self {
        let entry_function = entry_function.specialize(&spec.filter(shape.domains_dyn()));
        Module {
            entry_point: entry_function.id.clone(),
            struct_definitions: Default::default(),
            function_definitions: vec![entry_function],
            arguments: Default::default(),
            hash: shape.erased_hash(),
        }
    }

    pub fn with_args(mut self, args: impl IntoIterator<Item = Expr>) -> Self {
        self.arguments = args.into_iter().collect();
        self
    }

    pub fn call(&self, input: Expr) -> Expr {
        let mut args = self.arguments.clone();
        args.push(input);
        Expr::Call {
            function: self.entry_point.clone(),
            args,
        }
    }

    pub fn concat(self, rhs: Module) -> Module {
        // Aggregate function definitions
        let function_definitions: Vec<_> = self
            .function_definitions
            .into_iter()
            .chain(rhs.function_definitions)
            .collect();

        // Aggregate struct definitions
        let struct_definitions: Vec<_> = self
            .struct_definitions
            .into_iter()
            .chain(rhs.struct_definitions)
            .collect();

        // Done
        Module {
            function_definitions,
            struct_definitions,
            entry_point: rhs.entry_point,
            arguments: rhs.arguments,
            hash: rhs.hash,
        }
    }

    pub fn finalize(mut self) -> Module {
        let mut props = IndexSet::<PropertyIdentifier, RandomState>::default();
        for function in self.function_definitions.iter() {
            props.extend(block_props(&function.block));
        }

        // Deduplicate function definitions
        let mut set = std::collections::HashSet::new();
        self.function_definitions
            .retain(|x| set.insert(x.id.clone()));

        // Deduplicate struct definitions
        let mut set = std::collections::HashSet::new();
        self.struct_definitions.retain(|x| set.insert(x.id.clone()));

        self.struct_definitions = BUILTIN_STRUCTS
            .into_iter()
            .map(|def| (**def).clone())
            .chain(self.struct_definitions)
            .chain(StructDefinition {
                id: StructIdentifier(CONTEXT),
                public: true,
                fields: Cow::Owned(
                    props
                        .into_iter()
                        .map(|id| FieldDefinition { id, public: true })
                        .collect(),
                ),
            })
            .collect();

        self
    }
}

pub fn block_props(block: &Block) -> Vec<PropertyIdentifier> {
    let mut props = IndexSet::<PropertyIdentifier, RandomState>::default();
    for stmt in block.0.iter() {
        props.extend(stmt_props(stmt));
    }
    props.into_iter().collect()
}

fn stmt_props(stmt: &Stmt) -> Vec<PropertyIdentifier> {
    match stmt {
        Stmt::Block(block) => block.0.iter().flat_map(stmt_props).collect(),
        Stmt::Bind { expr, .. } => expr_props(expr),
        Stmt::Write { path, expr } => {
            let mut iter = path.iter();
            let path_props = if let Some(first) = iter.next() {
                if **first == CONTEXT {
                    iter.cloned().take(1).collect()
                } else if **first == COMBINE_CONTEXT {
                    iter.cloned().skip(1).take(1).collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };
            path_props.iter().cloned().chain(expr_props(expr)).collect()
        }
        Stmt::If {
            cond,
            then,
            otherwise,
        } => expr_props(cond)
            .into_iter()
            .chain(stmt_props(then))
            .chain(
                otherwise
                    .into_iter()
                    .flat_map(|otherwise| stmt_props(otherwise)),
            )
            .collect(),
        Stmt::Loop { stmt } => stmt_props(stmt),
        Stmt::Break => vec![],
        Stmt::Output(expr) => expr_props(expr),
    }
}

fn expr_props(expr: &Expr) -> Vec<PropertyIdentifier> {
    match expr {
        Expr::Struct(_, members) => members.values().flat_map(expr_props).collect(),
        Expr::Read(path) => {
            let mut iter = path.iter();
            if let Some(first) = iter.next() {
                if **first == CONTEXT {
                    return iter.cloned().take(1).collect();
                }
            }

            vec![]
        }
        Expr::Call { args, .. } => args.iter().flat_map(expr_props).collect(),
        Expr::Neg(expr)
        | Expr::Abs(expr)
        | Expr::Sign(expr)
        | Expr::Length(expr)
        | Expr::Normalize(expr) => expr_props(expr),
        Expr::Add(lhs, rhs)
        | Expr::Sub(lhs, rhs)
        | Expr::Mul(lhs, rhs)
        | Expr::Div(lhs, rhs)
        | Expr::Min(lhs, rhs)
        | Expr::Max(lhs, rhs)
        | Expr::Lt(lhs, rhs)
        | Expr::Gt(lhs, rhs)
        | Expr::Dot(lhs, rhs) => expr_props(lhs).into_iter().chain(expr_props(rhs)).collect(),
        Expr::Mix(lhs, rhs, t) => expr_props(lhs)
            .into_iter()
            .chain(expr_props(rhs))
            .chain(expr_props(t))
            .collect(),
        _ => vec![],
    }
}

pub trait AsModule {
    /// Generate a Module from this implementor
    fn module(&self, spec: &SpecializationData) -> Module;
}
