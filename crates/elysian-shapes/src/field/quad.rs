use std::hash::{Hash, Hasher};

use elysian_core::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, UV, VECTOR2, X, Y, Z},
    module::{
        AsModule, Domains, DomainsDyn, FunctionIdentifier, Module, SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::elysian_stmt;

use crate::{mirror::IntoMirror, modify::IntoTranslate, shape::Shape};

use super::Corner;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quad {
    extent: Expr,
    props: Vec<PropertyIdentifier>,
}

impl Quad {
    pub fn new(
        extent: impl IntoExpr,
        props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>,
    ) -> Self {
        Quad {
            extent: extent.expr(),
            props: props.into_iter().map(Into::into).collect(),
        }
    }
}

impl Hash for Quad {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.extent.hash(state);
        self.props.hash(state);
    }
}

impl Domains for Quad {
    fn domains() -> Vec<PropertyIdentifier> {
        Corner::domains()
    }
}

impl AsModule for Quad {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let one = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => Expr::vector2(1.0, 1.0),
            (false, true) => Expr::vector3(1.0, 1.0, 1.0),
            _ => panic!("Invalid position domain"),
        };

        let field = Corner::new(self.props.clone())
            .translate(self.extent.clone())
            .mirror_basis(one);

        let field_module = field.module(&spec.filter(field.domains_dyn()));
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let quad = FunctionIdentifier::new_dynamic("quad".into());

        field_module.concat(Module::new(
            self,
            spec,
            elysian_function! {
                fn quad(CONTEXT) -> CONTEXT {
                    let POSITION_3D = CONTEXT.POSITION_3D;

                    let CONTEXT = #field_call;

                    let mut X = 0.0;
                    let mut Y = 0.0;

                    if POSITION_3D.X.abs() >= POSITION_3D.Y.abs()
                    && POSITION_3D.X.abs() >= POSITION_3D.Z.abs()
                    {
                        X = POSITION_3D.Z;
                        Y = POSITION_3D.Y;
                    }
                    else {
                        if POSITION_3D.Y.abs() >= POSITION_3D.X.abs()
                        && POSITION_3D.Y.abs() >= POSITION_3D.Z.abs()
                        {
                            X = POSITION_3D.X;
                            Y = -POSITION_3D.Z;
                        }
                        else {
                            if POSITION_3D.Z.abs() >= POSITION_3D.X.abs()
                                && POSITION_3D.Z.abs() >= POSITION_3D.Y.abs()
                            {
                                X = POSITION_3D.X;
                                Y = POSITION_3D.Y;
                            }
                        }
                    }

                    CONTEXT.UV = VECTOR2 {
                        X: X,
                        Y: Y,
                    };

                    return CONTEXT;
                }
            },
        ))
    }
}

#[typetag::serde]
impl Shape for Quad {}
