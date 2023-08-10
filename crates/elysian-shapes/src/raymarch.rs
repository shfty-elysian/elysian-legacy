use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::expr::IntoExpr,
    ir::{
        ast::{
            Expr, Identifier, IntoLiteral, Stmt, DISTANCE, MATRIX4, POSITION_2D, POSITION_3D,
            VECTOR3, VECTOR4, W, X, Y, Z,
        },
        module::{
            AsIR, DomainsDyn, DynAsIR, FunctionIdentifier, IntoAsIR, NumericType,
            SpecializationData, StructIdentifier, Type, CONTEXT,
        },
    },
    property,
};
use elysian_decl_macros::elysian_function;
use elysian_proc_macros::elysian_stmt;

pub const RAYMARCH: FunctionIdentifier = FunctionIdentifier::new("raymarch", 2862797821569013866);

pub const RAY_FROM_4: Identifier = Identifier::new("ray_from_4", 1327263451507152945);
property!(
    RAY_FROM_4,
    RAY_FROM_4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const RAY_TO_4: Identifier = Identifier::new("ray_to_4", 1818903141506024705);
property!(
    RAY_TO_4,
    RAY_TO_4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const RAY_FROM_3: Identifier = Identifier::new("ray_from_3", 7265576981511357785);
property!(
    RAY_FROM_3,
    RAY_FROM_3_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const RAY_TO_3: Identifier = Identifier::new("ray_to_3", 5483986142139922358);
property!(
    RAY_TO_3,
    RAY_TO_3_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const RAY_POS: Identifier = Identifier::new("ray_pos", 203470946369255426);
property!(
    RAY_POS,
    RAY_POS_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const RAY_DIR: Identifier = Identifier::new("ray_dir", 11883607992066663879);
property!(
    RAY_DIR,
    RAY_DIR_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const T: Identifier = Identifier::new("t", 93144116760520780);
property!(T, T_PROP, Type::Number(NumericType::Float));

pub const INV_PROJECTION: Identifier = Identifier::new("inv_proj", 1835117139336577900);
property!(
    INV_PROJECTION,
    INV_PROJECTION_PROP,
    Type::Struct(StructIdentifier(MATRIX4))
);

pub const STEP_SIZE: Identifier = Identifier::new("step_size", 7777887281564637643);
property!(STEP_SIZE, STEP_SIZE_PROP, Type::Number(NumericType::UInt));

pub const EPSILON: Identifier = Identifier::new("epsilon", 32338215630771851);
property!(EPSILON, EPSILON_PROP, Type::Number(NumericType::Float));

pub const FRAC_1_K: Identifier = Identifier::new("frac_1_k", 5512322721559903899);
property!(FRAC_1_K, FRAC_1_K_PROP, Type::Number(NumericType::Float));

pub const CANDIDATE: Identifier = Identifier::new("candidate", 1956157168917067266);
property!(
    CANDIDATE,
    CANDIDATE_PROP,
    Type::Struct(StructIdentifier(CONTEXT))
);

pub const STEPS: Identifier = Identifier::new("steps", 1682585060223888912);
property!(STEPS, STEPS_PROP, Type::Number(NumericType::UInt));

pub const MAX_STEPS: Identifier = Identifier::new("max_steps", 1146747975614382616);
property!(MAX_STEPS, MAX_STEPS_PROP, Type::Number(NumericType::UInt));

#[derive(Debug)]
pub enum March {
    Fixed {
        step_size: elysian_core::ast::expr::Expr,
    },
    Sphere {
        epsilon: elysian_core::ast::expr::Expr,
    },
    Lipschitz {
        epsilon: elysian_core::ast::expr::Expr,
        falloff_k: elysian_core::ast::expr::Expr,
    },
}

pub fn falloff_k(e: f32, r: f32) -> f32 {
    1.72 * e.abs() / r
}

#[derive(Debug)]
pub struct Raymarch {
    march: March,
    max_steps: elysian_core::ast::expr::Expr,
    inv_projection: elysian_core::ast::expr::Expr,
    field: DynAsIR,
}

impl Raymarch {
    pub fn fixed(
        step_size: impl IntoExpr,
        max_steps: impl IntoExpr,
        inv_projection: impl IntoExpr,
        field: impl IntoAsIR,
    ) -> Self {
        Raymarch {
            march: March::Fixed {
                step_size: step_size.expr(),
            },
            max_steps: max_steps.expr(),
            inv_projection: inv_projection.expr(),
            field: field.as_ir(),
        }
    }

    pub fn sphere(
        epsilon: impl IntoExpr,
        max_steps: impl IntoExpr,
        inv_projection: impl IntoExpr,
        field: impl IntoAsIR,
    ) -> Self {
        Raymarch {
            march: March::Sphere {
                epsilon: epsilon.expr(),
            },
            max_steps: max_steps.expr(),
            inv_projection: inv_projection.expr(),
            field: field.as_ir(),
        }
    }

    pub fn lipschitz(
        epsilon: impl IntoExpr,
        falloff_k: impl IntoExpr,
        max_steps: impl IntoExpr,
        inv_projection: impl IntoExpr,
        field: impl IntoAsIR,
    ) -> Self {
        Raymarch {
            march: March::Lipschitz {
                epsilon: epsilon.expr(),
                falloff_k: falloff_k.expr(),
            },
            max_steps: max_steps.expr(),
            inv_projection: inv_projection.expr(),
            field: field.as_ir(),
        }
    }
}

impl Hash for Raymarch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl DomainsDyn for Raymarch {
    fn domains_dyn(&self) -> Vec<elysian_core::ir::module::PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsIR for Raymarch {
    fn entry_point(&self) -> FunctionIdentifier {
        RAYMARCH
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        if !spec.contains(&POSITION_2D.into()) {
            panic!("Raymarch is only compatible with the 2D Position domain");
        }

        if !spec.contains(&DISTANCE.into()) {
            panic!("Raymarch requires the Distance domain");
        }

        let spec_3d = SpecializationData::new_3d();
        let (_, field_call, field_functions) = self.field.call(&spec_3d, elysian_stmt! { CONTEXT });

        let max_steps = Expr::from(self.max_steps.clone());
        let inv_projection = Expr::from(self.inv_projection.clone());
        let f32_max = f32::MAX.literal();

        let march = match &self.march {
            March::Fixed { step_size } => {
                let step_size = Expr::from(step_size.clone());
                elysian_stmt! {
                    T = T + #step_size
                }
            }
            March::Sphere { epsilon } => {
                let epsilon = Expr::from(epsilon.clone());
                elysian_stmt! {
                    T = T + #epsilon.max(CONTEXT.DISTANCE.abs())
                }
            }
            March::Lipschitz { epsilon, falloff_k } => {
                let epsilon = Expr::from(epsilon.clone());
                let falloff_k = Expr::from(falloff_k.clone());
                elysian_stmt! {
                    T = T + #epsilon.max(CONTEXT.DISTANCE.abs() * #falloff_k)
                }
            }
        };

        field_functions
            .into_iter()
            .chain([elysian_function! {
                fn entry_point(mut CONTEXT) -> CONTEXT {
                    let MAX_STEPS = #max_steps;
                    let STEPS = 0u32;

                    let INV_PROJECTION = #inv_projection;

                    let RAY_FROM_4 = INV_PROJECTION * VECTOR4 {
                        X: CONTEXT.POSITION_2D.X,
                        Y: CONTEXT.POSITION_2D.Y,
                        Z: 0.0,
                        W: 1.0,
                    };
                    let RAY_TO_4 = INV_PROJECTION * VECTOR4 {
                        X: CONTEXT.POSITION_2D.X,
                        Y: CONTEXT.POSITION_2D.Y,
                        Z: -1.0,
                        W: 1.0,
                    };

                    let RAY_FROM_3 = VECTOR3 {
                        X: RAY_FROM_4.X / RAY_FROM_4.W,
                        Y: RAY_FROM_4.Y / RAY_FROM_4.W,
                        Z: RAY_FROM_4.Z / RAY_FROM_4.W,
                    };
                    let RAY_TO_3 = VECTOR3 {
                        X: RAY_TO_4.X / RAY_TO_4.W,
                        Y: RAY_TO_4.Y / RAY_TO_4.W,
                        Z: RAY_TO_4.Z / RAY_TO_4.W,
                    };

                    let RAY_DIR = (RAY_FROM_3 - RAY_TO_3).normalize();

                    CONTEXT.DISTANCE = #f32_max;
                    let T = 0.0;

                    loop {
                        let RAY_POS = RAY_FROM_3 + RAY_DIR * T;
                        CONTEXT.POSITION_3D = RAY_POS;
                        let CANDIDATE = #field_call;

                        if CANDIDATE.DISTANCE < CONTEXT.DISTANCE {
                            CONTEXT = CANDIDATE;
                        }

                        STEPS = STEPS + 1u32;

                        if CONTEXT.DISTANCE < 0.0 {
                            break;
                        }

                        if STEPS > MAX_STEPS {
                            break;
                        }

                        #march
                    }

                    return CONTEXT;
                }
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
