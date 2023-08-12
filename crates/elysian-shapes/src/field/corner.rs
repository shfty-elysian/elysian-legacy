use std::hash::Hash;

use crate::combine::{Combinator, COMBINE_CONTEXT_STRUCT};
use elysian_core::{
    ast::{expr::Expr, property_identifier::PropertyIdentifier},
    ir::{
        ast::{DISTANCE, POSITION_2D, POSITION_3D},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, SpecializationData, CONTEXT,
        },
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

impl AsIR for Corner {
    fn entry_point(&self) -> FunctionIdentifier {
        CORNER
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let zero = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => Expr::vector2(0.0, 0.0),
            (false, true) => Expr::vector3(0.0, 0.0, 0.0),
            _ => panic!("Invalid position domain"),
        };

        let combinator = Combinator::build()
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

        let (_, field_call, field_functions) = field.call(spec, elysian_stmt! { CONTEXT });

        field_functions
            .into_iter()
            .chain([elysian_function! {
                fn entry_point(CONTEXT) -> CONTEXT {
                    return #field_call;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        vec![COMBINE_CONTEXT_STRUCT.clone()]
    }
}
