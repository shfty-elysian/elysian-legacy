use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashSet},
    fmt::Debug,
    sync::OnceLock,
};

use indexmap::{IndexMap, IndexSet};
use uuid::Uuid;

use crate::ir::{
    as_ir::HashIR,
    ast::{
        Block, Expr, Identifier, Property, Stmt, MATRIX2_STRUCT, MATRIX3_STRUCT, MATRIX4_STRUCT,
        VECTOR2_STRUCT, VECTOR3_STRUCT, VECTOR4_STRUCT,
    },
    module::FieldDefinition,
};

use super::{
    FunctionDefinition, FunctionIdentifier, Module, PropertyIdentifier, SpecializationData,
    StructDefinition, StructIdentifier, Type,
};

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

fn stmt_props(stmt: &Stmt) -> Vec<PropertyIdentifier> {
    match stmt {
        Stmt::Block(block) => block.0.iter().flat_map(stmt_props).collect(),
        Stmt::Bind { expr, .. } => expr_props(expr),
        Stmt::Write { path, expr } => {
            let mut iter = path.iter();
            let path_props = if let Some(first) = iter.next() {
                if **first == CONTEXT {
                    iter.cloned().take(1).collect()
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

fn block_props(block: &Block) -> Vec<PropertyIdentifier> {
    let mut props = IndexSet::<PropertyIdentifier, RandomState>::default();
    for stmt in block.0.iter() {
        props.extend(stmt_props(stmt));
    }
    props.into_iter().collect()
}

#[macro_export]
macro_rules! property {
    ($id:ident, $prop:ident, $ty:expr) => {
        #[linkme::distributed_slice(elysian_core::ir::module::PROPERTIES)]
        static $prop: elysian_core::ir::ast::Property =
            elysian_core::ir::ast::Property { id: $id, ty: $ty };
    };
}

pub const CONTEXT: Identifier = Identifier::new("Context", 595454262490629935);
pub const CONTEXT_PROP: PropertyIdentifier = PropertyIdentifier(CONTEXT);
property!(
    CONTEXT_PROP,
    CONTEXT_PROP_DEF,
    Type::Struct(StructIdentifier(CONTEXT))
);

/// Distributed slice of Identifier -> Type pairs
#[linkme::distributed_slice]
pub static PROPERTIES: [Property] = [..];

pub static PROPERTIES_MAP: OnceLock<IndexMap<PropertyIdentifier, Type>> = OnceLock::new();

pub fn properties() -> &'static IndexMap<PropertyIdentifier, Type> {
    PROPERTIES_MAP.get_or_init(|| {
        let props: IndexMap<_, _> = PROPERTIES
            .into_iter()
            .map(|prop| (prop.id.clone(), prop.ty.clone()))
            .collect();

        for (i, (id, _)) in props.iter().enumerate() {
            if let Some((_, (cand, _))) =
                props
                    .iter()
                    .enumerate()
                    .filter(|(u, _)| i != *u)
                    .find(|(_, (cand, _))| {
                        let id = id.uuid();
                        let cand = cand.uuid();
                        let nil = Uuid::nil();
                        if *id == nil && *cand == nil {
                            return false;
                        }

                        id == cand
                    })
            {
                panic!(
                    "Properties: UUID Collision between {} and {}",
                    cand.name(),
                    id.name()
                )
            }
        }

        props
    })
}

pub trait AsModule: 'static + Debug + HashIR {
    fn module(&self, spec: &SpecializationData) -> Module {
        let types: IndexMap<_, _> = properties().clone();

        let entry_point = self.entry_point();
        let mut functions = self.functions(spec, &types, &entry_point);

        let mut set = HashSet::new();
        functions.retain(|x| set.insert(x.id.clone()));

        let mut props = IndexSet::<PropertyIdentifier, RandomState>::default();
        for function in functions.iter() {
            props.extend(block_props(&function.block));
        }

        let context_struct = StructDefinition {
            id: StructIdentifier(CONTEXT),
            public: true,
            fields: Cow::Owned(
                props
                    .into_iter()
                    .map(|id| FieldDefinition { id, public: true })
                    .collect(),
            ),
        };

        let struct_definitions = vec![
            VECTOR2_STRUCT.clone(),
            VECTOR3_STRUCT.clone(),
            VECTOR4_STRUCT.clone(),
            MATRIX2_STRUCT.clone(),
            MATRIX3_STRUCT.clone(),
            MATRIX4_STRUCT.clone(),
            context_struct,
        ]
        .into_iter()
        .chain(self.structs())
        .collect();

        Module {
            props: types,
            entry_point,
            struct_definitions,
            function_definitions: functions,
        }
    }

    fn entry_point(&self) -> FunctionIdentifier;

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<PropertyIdentifier, Type>,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition>;

    fn structs(&self) -> Vec<StructDefinition> {
        vec![]
    }
}

pub type DynAsModule = Box<dyn AsModule>;

impl AsModule for DynAsModule {
    fn module(&self, spec: &SpecializationData) -> Module {
        (**self).module(spec)
    }

    fn entry_point(&self) -> FunctionIdentifier {
        (**self).entry_point()
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<PropertyIdentifier, Type>,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        (**self).functions(spec, tys, entry_point)
    }

    fn structs(&self) -> Vec<StructDefinition> {
        (**self).structs()
    }
}

impl HashIR for DynAsModule {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}
