use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        ast::{POSITION_2D, POSITION_3D},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData, CONTEXT,
        },
    },
};
use elysian_decl_macros::elysian_function;

use crate::modify::{ClampMode, Elongate, DIR_2D, DIR_3D};

use super::Point;

pub const LINE: FunctionIdentifier = FunctionIdentifier::new("line", 14339483921749952476);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LineMode {
    Centered,
    Segment,
}

impl ToString for LineMode {
    fn to_string(&self) -> String {
        match self {
            LineMode::Centered => "centered",
            LineMode::Segment => "segment",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub dir: Expr,
    pub mode: LineMode,
}

impl Hash for Line {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        LINE.uuid().hash(state);
        self.mode.hash(state);
        self.dir.hash(state);
    }
}

impl Domains for Line {
    fn domains() -> Vec<PropertyIdentifier> {
        Point::domains()
            .into_iter()
            .chain(Elongate::domains())
            .collect()
    }
}

impl AsIR for Line {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        LINE.concat(&FunctionIdentifier::new_dynamic(
            self.mode.to_string().into(),
        ))
        .specialize(spec)
    }

    fn arguments(&self, input: elysian_core::ir::ast::Expr) -> Vec<elysian_core::ir::ast::Expr> {
        vec![self.dir.clone().into(), input]
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain set")
        };

        let point = Point;
        let (_, point_entry, point_functions) = point.prepare(spec);

        let clamp_neg = match self.mode {
            LineMode::Centered => ClampMode::Dir,
            LineMode::Segment => ClampMode::Zero,
        };

        let elongate = Elongate {
            dir: self.dir.clone(),
            clamp_neg,
            clamp_pos: ClampMode::Dir,
        };
        let (_, elongate_entry, elongate_functions) = elongate.prepare(spec);

        point_functions
            .into_iter()
            .chain(elongate_functions)
            .chain(elysian_function! {
                fn entry_point(dir, CONTEXT) -> CONTEXT {
                    return point_entry(elongate_entry(dir, CONTEXT));
                }
            })
            .collect()
    }
}
