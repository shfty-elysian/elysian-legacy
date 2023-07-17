use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ast::expr::Expr;
use crate::ast::post_modifier::isosurface::{Isosurface, ISOSURFACE};

use crate::ast::field::Line;
use crate::ast::pre_modifier::elongate::DIR;
use crate::ir::as_ir::AsIR;
use crate::ir::ast::{Identifier, IntoBlock, CONTEXT};
use crate::ir::from_elysian::CONTEXT_STRUCT;
use crate::ir::module::{FunctionDefinition, InputDefinition};

use super::{LINE, RADIUS};

pub const CAPSULE: Identifier = Identifier::new("capsule", 14339483921749952476);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Capsule<N, V> {
    pub dir: Expr<N, V>,
    pub radius: Expr<N, V>,
}

impl<N, V> Hash for Capsule<N, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state);
        self.radius.hash(state);
    }
}

impl<N, V> AsIR<N, V> for Capsule<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
        Line {
            dir: self.dir.clone(),
        }
        .functions()
        .into_iter()
        .chain(
            Isosurface {
                dist: self.radius.clone(),
            }
            .functions(),
        )
        .chain([FunctionDefinition {
            id: CAPSULE,
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: DIR,
                    mutable: false,
                },
                InputDefinition {
                    prop: RADIUS,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                },
            ],
            output: CONTEXT_STRUCT,
            block: [crate::ir::ast::Expr::Call {
                function: ISOSURFACE,
                args: vec![
                    RADIUS.read(),
                    crate::ir::ast::Expr::Call {
                        function: LINE,
                        args: vec![DIR.read(), CONTEXT.read()],
                    },
                ],
            }
            .output()]
            .block(),
        }])
        .collect()
    }

    fn expression(&self, input: crate::ir::ast::Expr<N, V>) -> crate::ir::ast::Expr<N, V> {
        crate::ir::ast::Expr::Call {
            function: CAPSULE,
            args: vec![self.dir.clone().into(), self.radius.clone().into(), input],
        }
    }
}
