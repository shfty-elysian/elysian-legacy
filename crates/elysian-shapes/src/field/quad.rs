use std::hash::{Hash, Hasher};

use elysian_core::{
    expr::{Expr, IntoExpr},
    property_identifier::PropertyIdentifier,
};
use elysian_ir::{
    ast::{POSITION_2D, POSITION_3D, UV, VECTOR2, X, Y, Z},
    module::{
        AsModule, Domains, DomainsDyn, FunctionDefinition, FunctionIdentifier, InputDefinition,
        Module, SpecializationData, CONTEXT,
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::{modify::IntoTranslate, shape::Shape, wrap::mirror::IntoMirror};

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
        let (position, one) = match (
            spec.contains(&POSITION_2D.into()),
            spec.contains(&POSITION_3D.into()),
        ) {
            (true, false) => (POSITION_2D, Expr::vector2(1.0, 1.0)),
            (false, true) => (POSITION_2D, Expr::vector3(1.0, 1.0, 1.0)),
            _ => panic!("Invalid position domain"),
        };

        let field = Corner::new(self.props.clone())
            .translate(self.extent.clone())
            .mirror_basis(one);

        let field_module = field.module(&spec.filter(field.domains_dyn()));
        let field_call = field_module.call(elysian_stmt! { CONTEXT });

        let quad = FunctionIdentifier::new_dynamic("quad".into());

        let mut block = elysian_block! {
            let position = CONTEXT.position;

            let CONTEXT = #field_call;
        };

        if spec.contains(&UV.into()) {
            block.extend(match &position {
                p if *p == POSITION_2D => elysian_block! {
                    CONTEXT.UV = CONTEXT.POSITION_2D;
                },
                p if *p == POSITION_3D => elysian_block! {
                    let mut X = 0.0;
                    let mut Y = 0.0;

                    if position.X.abs() >= position.Y.abs()
                    && position.X.abs() >= position.Z.abs()
                    {
                        X = position.Z;
                        Y = position.Y;
                    }
                    else {
                        if position.Y.abs() >= position.X.abs()
                        && position.Y.abs() >= position.Z.abs()
                        {
                            X = position.X;
                            Y = -position.Z;
                        }
                        else {
                            if position.Z.abs() >= position.X.abs()
                                && position.Z.abs() >= position.Y.abs()
                            {
                                X = position.X;
                                Y = position.Y;
                            }
                        }
                    }

                    CONTEXT.UV = VECTOR2 {
                        X: X,
                        Y: Y,
                    };
                },
                _ => unreachable!(),
            });
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        field_module.concat(Module::new(
            self,
            spec,
            FunctionDefinition {
                id: quad,
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: false,
                }],
                output: CONTEXT.into(),
                block,
            },
        ))
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Quad {}
