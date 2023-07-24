use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashSet},
    fmt::Debug,
};

use indexmap::IndexSet;

use crate::ir::{
    as_ir::HashIR,
    ast::{Block, Expr, Identifier, Property, Stmt, CONTEXT},
    module::FieldDefinition,
};

use super::{FunctionDefinition, Module, SpecializationData, StructDefinition};

fn expr_props(expr: &Expr) -> Vec<Property> {
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

fn stmt_props(stmt: &Stmt) -> Vec<Property> {
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

fn block_props(block: &Block) -> Vec<Property> {
    let mut props = IndexSet::<Property, RandomState>::default();
    for stmt in block.0.iter() {
        props.extend(stmt_props(stmt));
    }
    props.into_iter().collect()
}

pub trait AsModule: 'static + Debug + HashIR {
    fn module(&self, spec: &SpecializationData) -> Module {
        let entry_point = self.entry_point();
        let mut functions = self.functions(spec, &entry_point);

        let mut set = HashSet::new();
        functions.retain(|x| set.insert(x.id.clone()));

        let mut props = IndexSet::<Property, RandomState>::default();
        for function in functions.iter() {
            props.extend(block_props(&function.block));
        }

        let context_struct = StructDefinition {
            id: CONTEXT.id().clone(),
            public: true,
            fields: Cow::Owned(
                props
                    .into_iter()
                    .map(|prop| FieldDefinition { prop, public: true })
                    .collect(),
            ),
        };

        Module {
            entry_point,
            struct_definitions: self.structs(),
            function_definitions: functions,
        }
    }

    fn entry_point(&self) -> Identifier;
    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition>;
    fn structs(&self) -> Vec<StructDefinition>;
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
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        (**self).functions(spec, entry_point)
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
