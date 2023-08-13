use std::hash::Hash;

use crate::combine::{CombineBuilder, COMBINE_CONTEXT_STRUCT};
use elysian_core::{expr::Expr, property_identifier::PropertyIdentifier};
use elysian_ir::{
    ast::{DISTANCE, POSITION_2D, POSITION_3D},
    module::{
        AsModule, Domains, DomainsDyn, FunctionIdentifier, Module, SpecializationData, CONTEXT,
    },
};

use elysian_decl_macros::elysian_function;
use elysian_proc_macros::elysian_stmt;

use crate::{
    combine::{Displace, Sided, SidedProp},
    modify::{BoundType, IntoBasisBound, IntoDistanceBound},
};

use super::{Chebyshev, Point};

pub const CORNER: FunctionIdentifier = FunctionIdentifier::new("corner", 5817708551492744655);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Corner {
    props: Vec<PropertyIdentifier>,
}

impl Corner {
    pub fn new(props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Self {
        Corner {
            props: props.into_iter().map(Into::into).collect(),
        }
    }
}

impl Hash for Corner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        CORNER.uuid().hash(state);
    }
}

impl Domains for Corner {
    fn domains() -> Vec<PropertyIdentifier> {
        Point::domains()
            .into_iter()
            .chain(Chebyshev::domains())
            .collect()
    }
}

impl AsModule for Corner {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let zero = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => Expr::vector2(0.0, 0.0),
            (false, true) => Expr::vector3(0.0, 0.0, 0.0),
            _ => panic!("Invalid position domain"),
        };

        let combinator = CombineBuilder::build()
            .push(Sided::left())
            .push(Displace::new(DISTANCE));

        let field = self
            .props
            .iter()
            .fold(combinator, |acc, next| {
                acc.push(SidedProp::new(next.clone(), false))
            })
            .combine()
            .push(Point.basis_bound(BoundType::Lower, zero))
            .push(Chebyshev.distance_bound(BoundType::Upper, 0.0));

        let field_module = field.module_impl(&spec.filter(field.domains_dyn()));
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let mut corner_module = Module::new(
            self,
            spec,
            elysian_function! {
                fn CORNER(CONTEXT) -> CONTEXT {
                    return #field_call;
                }
            },
        );

        corner_module
            .struct_definitions
            .push(COMBINE_CONTEXT_STRUCT.clone());

        field_module.concat(corner_module)
    }
}
