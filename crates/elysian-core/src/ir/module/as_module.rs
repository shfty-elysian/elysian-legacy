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
        Block, Expr, Identifier, Property, Stmt, CONTEXT, MATRIX2, MATRIX2_STRUCT, MATRIX3,
        MATRIX3_STRUCT, MATRIX4, MATRIX4_STRUCT, VECTOR2, VECTOR2_STRUCT, VECTOR3, VECTOR3_STRUCT,
        VECTOR4, VECTOR4_STRUCT,
    },
    module::FieldDefinition,
};

use super::{FunctionDefinition, Module, SpecializationData, StructDefinition, Type};

fn expr_props(expr: &Expr) -> Vec<Identifier> {
    match expr {
        Expr::Struct(_, members) => members.values().flat_map(expr_props).collect(),
        Expr::Read(path) => {
            let mut iter = path.iter();
            if let Some(first) = iter.next() {
                if *first == CONTEXT {
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

fn stmt_props(stmt: &Stmt) -> Vec<Identifier> {
    match stmt {
        Stmt::Block(block) => block.0.iter().flat_map(stmt_props).collect(),
        Stmt::Bind { expr, .. } => expr_props(expr),
        Stmt::Write { path, expr } => {
            let mut iter = path.iter();
            let path_props = if let Some(first) = iter.next() {
                if *first == CONTEXT {
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

fn block_props(block: &Block) -> Vec<Identifier> {
    let mut props = IndexSet::<Identifier, RandomState>::default();
    for stmt in block.0.iter() {
        props.extend(stmt_props(stmt));
    }
    props.into_iter().collect()
}

/// Distributed slice of Identifier -> Type pairs
#[linkme::distributed_slice]
pub static PROPERTIES: [Property] = [..];

pub static PROPERTIES_MAP: OnceLock<IndexMap<Identifier, Type>> = OnceLock::new();

pub fn properties() -> &'static IndexMap<Identifier, Type> {
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

#[linkme::distributed_slice(PROPERTIES)]
static VECTOR2_PROP: Property = Property {
    id: VECTOR2,
    ty: Type::Struct(VECTOR2),
};

#[linkme::distributed_slice(PROPERTIES)]
static VECTOR3_PROP: Property = Property {
    id: VECTOR3,
    ty: Type::Struct(VECTOR3),
};

#[linkme::distributed_slice(PROPERTIES)]
static VECTOR4_PROP: Property = Property {
    id: VECTOR4,
    ty: Type::Struct(VECTOR4),
};

#[linkme::distributed_slice(PROPERTIES)]
static MATRIX2_PROP: Property = Property {
    id: MATRIX2,
    ty: Type::Struct(MATRIX2),
};

#[linkme::distributed_slice(PROPERTIES)]
static MATRIX3_PROP: Property = Property {
    id: MATRIX3,
    ty: Type::Struct(MATRIX3),
};

#[linkme::distributed_slice(PROPERTIES)]
static MATRIX4_PROP: Property = Property {
    id: MATRIX4,
    ty: Type::Struct(MATRIX4),
};

pub trait AsModule: 'static + Debug + HashIR {
    fn module(&self, spec: &SpecializationData) -> Module {
        let types: IndexMap<_, _> = properties().clone();

        let entry_point = self.entry_point();
        let mut functions = self.functions(spec, &types, &entry_point);

        let mut set = HashSet::new();
        functions.retain(|x| set.insert(x.id.clone()));

        let mut props = IndexSet::<Identifier, RandomState>::default();
        for function in functions.iter() {
            props.extend(block_props(&function.block));
        }

        let context_struct = StructDefinition {
            id: CONTEXT,
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
            types,
            entry_point,
            struct_definitions,
            function_definitions: functions,
        }
    }

    fn entry_point(&self) -> Identifier;

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<Identifier, Type>,
        entry_point: &Identifier,
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

    fn entry_point(&self) -> Identifier {
        (**self).entry_point()
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<Identifier, Type>,
        entry_point: &Identifier,
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
