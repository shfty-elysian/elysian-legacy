use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D},
    module::{
        AsModule, Domains, DomainsDyn, FunctionIdentifier, Module, SpecializationData,
        CONTEXT,
    },
};
use elysian_proc_macros::elysian_stmt;

use crate::modify::{ClampMode, ElongateAxis, DIR_2D, DIR_3D};

use super::Point;

pub const LINE: FunctionIdentifier = FunctionIdentifier::new("line", 14339483921749952476);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Line {
    dir: Expr,
    mode: LineMode,
}

impl Hash for Line {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        LINE.uuid().hash(state);
        self.mode.hash(state);
        self.dir.hash(state);
    }
}

impl Line {
    pub fn segment(dir: impl IntoExpr) -> Self {
        Line {
            dir: dir.expr(),
            mode: LineMode::Segment,
        }
    }

    pub fn centered(dir: impl IntoExpr) -> Self {
        Line {
            dir: dir.expr(),
            mode: LineMode::Centered,
        }
    }
}

impl Domains for Line {
    fn domains() -> Vec<PropertyIdentifier> {
        Point::domains()
            .into_iter()
            .chain(ElongateAxis::domains())
            .collect()
    }
}

impl AsModule for Line {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let dir = if spec.contains(&POSITION_2D.into()) {
            DIR_2D
        } else if spec.contains(&POSITION_3D.into()) {
            DIR_3D
        } else {
            panic!("No position domain set")
        };

        let elongate = ElongateAxis {
            dir: self.dir.clone(),
            clamp_neg: match self.mode {
                LineMode::Centered => ClampMode::Dir,
                LineMode::Segment => ClampMode::Zero,
            },
            clamp_pos: ClampMode::Dir,
        };
        let elongate_module = elongate.module_impl(&spec.filter(elongate.domains_dyn()));
        let elongate_call = elongate_module.call(elysian_stmt! { CONTEXT });

        let point = Point;
        let point_module = point.module_impl(&spec.filter(point.domains_dyn()));
        let point_call = point_module.call(elongate_call);

        let line = LINE.concat(&FunctionIdentifier::new_dynamic(
            self.mode.to_string().into(),
        ));

        elongate_module.concat(point_module).concat(
            Module::new(
                self,
                spec,
                elysian_function! {
                    fn line(dir, CONTEXT) -> CONTEXT {
                        return #point_call;
                    }
                },
            )
            .with_args([self.dir.clone().into()]),
        )
    }
}
