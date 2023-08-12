use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashSet},
    fmt::Debug,
    sync::OnceLock,
};

use elysian_core::{
    ast::{identifier::Identifier, property_identifier::PropertyIdentifier},
    uuid::Uuid,
};
use indexmap::{IndexMap, IndexSet};

use crate::{
    ast::{
        Block, Expr, Property, Stmt, COMBINE_CONTEXT, MATRIX2_STRUCT, MATRIX3_STRUCT,
        MATRIX4_STRUCT, VECTOR2_STRUCT, VECTOR3_STRUCT, VECTOR4_STRUCT,
    },
    module::{FieldDefinition, HashIR},
};

use super::{
    DomainsDyn, FunctionDefinition, FunctionIdentifier, Module, SpecializationData,
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
        #[linkme::distributed_slice($crate::module::PROPERTIES)]
        static $prop: $crate::ast::Property = $crate::ast::Property {
            id: elysian_core::ast::property_identifier::PropertyIdentifier($id),
            ty: $ty,
        };
    };
}

pub const CONTEXT: Identifier = Identifier::new("Context", 595454262490629935);
property!(
    CONTEXT,
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

pub const SAFE_NORMALIZE_2: FunctionIdentifier =
    FunctionIdentifier::new("safe_normalize_2", 18883451341246143490);
pub const SAFE_NORMALIZE_3: FunctionIdentifier =
    FunctionIdentifier::new("safe_normalize_3", 174303162393329419);
pub const SAFE_NORMALIZE_4: FunctionIdentifier =
    FunctionIdentifier::new("safe_normalize_4", 18890028961074310202);

/// A type that can be converted into Elysian IR
pub trait AsIR: Debug + HashIR + DomainsDyn {
    /// Generate a Module from this implementor
    fn module(&self, spec: &SpecializationData) -> Module {
        let types: IndexMap<_, _> = properties().clone();

        let (_, entry_point, mut functions) = self.prepare(spec);

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

        let mut struct_definitions: Vec<_> = vec![
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

        let mut set = HashSet::new();
        struct_definitions.retain(|x| set.insert(x.id.clone()));

        Module {
            props: types,
            entry_point,
            struct_definitions,
            function_definitions: functions,
        }
    }

    /// Generate the function identifier used to reference this implementor
    fn entry_point(&self) -> FunctionIdentifier;

    /// Given a SpecializationData, filter it by this implementor's domain list
    fn filter_spec(&self, spec: &SpecializationData) -> SpecializationData {
        spec.filter(self.domains_dyn())
    }

    /// Generate a list of arguments used to call this implementor
    fn arguments(&self, input: Expr) -> Vec<Expr> {
        vec![input]
    }

    /// Given a spec and entry point, produce a list of function definitions
    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition>;

    /// Produce a list of struct definitions
    fn structs(&self) -> Vec<StructDefinition> {
        vec![]
    }

    /// Utility for preparing a filtered SpecializationData,
    /// and using it to produce an entry point and list of functions
    /// Useful when a function is called multiple times with different arguments.
    fn prepare(
        &self,
        spec: &SpecializationData,
    ) -> (
        SpecializationData,
        FunctionIdentifier,
        Vec<FunctionDefinition>,
    ) {
        let spec = self.filter_spec(spec);
        let entry_point = self.entry_point().specialize(&spec);
        let functions = self.functions(&spec, &entry_point);
        (spec, entry_point, functions)
    }

    /// Utility for prepareing a filtered Specialization data,
    /// and using it to produce a call statement and list of functions.
    /// Useful when a function is called with a singular set of arguments.
    fn call(
        &self,
        spec: &SpecializationData,
        input: Expr,
    ) -> (SpecializationData, Expr, Vec<FunctionDefinition>) {
        let (spec, entry, functions) = self.prepare(spec);
        (spec, entry.call(self.arguments(input)), functions)
    }
}

pub type DynAsIR = Box<dyn AsIR>;

impl DomainsDyn for DynAsIR {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        (**self).domains_dyn()
    }
}

impl AsIR for DynAsIR {
    fn module(&self, spec: &SpecializationData) -> Module {
        (**self).module(spec)
    }

    fn entry_point(&self) -> FunctionIdentifier {
        (**self).entry_point()
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        (**self).functions(spec, entry_point)
    }

    fn structs(&self) -> Vec<StructDefinition> {
        (**self).structs()
    }

    fn arguments(&self, input: Expr) -> Vec<Expr> {
        (**self).arguments(input)
    }
}

impl HashIR for DynAsIR {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}

pub trait IntoAsIR: 'static + Sized + AsIR {
    fn as_ir(self) -> DynAsIR {
        Box::new(self)
    }
}
impl<T> IntoAsIR for T where T: 'static + Sized + AsIR {}
