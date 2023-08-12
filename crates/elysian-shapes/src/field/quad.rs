use std::hash::{Hash, Hasher};

use elysian_core::ast::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, UV, VECTOR2, X, Y, Z},
    module::{AsIR, Domains, FunctionIdentifier, CONTEXT},
};
use elysian_proc_macros::elysian_expr;

use crate::modify::{IntoMirror, IntoTranslate};

use super::Corner;

#[derive(Debug)]
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

impl AsIR for Quad {
    fn entry_point(&self) -> elysian_ir::module::FunctionIdentifier {
        FunctionIdentifier::new_dynamic("quad".into())
    }

    fn functions(
        &self,
        spec: &elysian_ir::module::SpecializationData,
        entry_point: &elysian_ir::module::FunctionIdentifier,
    ) -> Vec<elysian_ir::module::FunctionDefinition> {
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

        let (_, field_call, field_functions) = field.call(spec, elysian_expr! { CONTEXT });

        field_functions
            .into_iter()
            .chain([elysian_function! {
                fn entry_point(CONTEXT) -> CONTEXT {
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
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_ir::module::StructDefinition> {
        Corner::new(self.props.clone()).structs()
    }
}
