use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::ir::{
    ast::{
        Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, Number, Property, Stmt,
        CONTEXT, DISTANCE, MATRIX4, POSITION_2D, POSITION_3D, VECTOR3, VECTOR4, W, X, Y, Z,
    },
    module::{
        AsModule, FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type,
        PROPERTIES,
    },
};

pub const RAYMARCH: Identifier = Identifier::new("raymarch", 2862797821569013866);

pub const RAY_FROM_4: Identifier = Identifier::new("ray_from_4", 1327263451507152945);
#[linkme::distributed_slice(PROPERTIES)]
static RAY_FROM_4_PROP: Property = Property {
    id: RAY_FROM_4,
    ty: Type::Struct(VECTOR4),
};

pub const RAY_TO_4: Identifier = Identifier::new("ray_to_4", 1818903141506024705);
#[linkme::distributed_slice(PROPERTIES)]
static RAY_TO_4_PROP: Property = Property {
    id: RAY_TO_4,
    ty: Type::Struct(VECTOR4),
};

pub const RAY_FROM_3: Identifier = Identifier::new("ray_from_3", 7265576981511357785);
#[linkme::distributed_slice(PROPERTIES)]
static RAY_FROM_3_PROP: Property = Property {
    id: RAY_FROM_3,
    ty: Type::Struct(VECTOR3),
};

pub const RAY_TO_3: Identifier = Identifier::new("ray_to_3", 5483986142139922358);
#[linkme::distributed_slice(PROPERTIES)]
static RAY_TO_3_PROP: Property = Property {
    id: RAY_TO_3,
    ty: Type::Struct(VECTOR3),
};

pub const RAY_POS: Identifier = Identifier::new("ray_pos", 203470946369255426);
#[linkme::distributed_slice(PROPERTIES)]
static RAY_POS_PROP: Property = Property {
    id: RAY_POS,
    ty: Type::Struct(VECTOR3),
};

pub const RAY_DIR: Identifier = Identifier::new("ray_dir", 11883607992066663879);
#[linkme::distributed_slice(PROPERTIES)]
static RAY_DIR_PROP: Property = Property {
    id: RAY_DIR,
    ty: Type::Struct(VECTOR3),
};

pub const T: Identifier = Identifier::new("t", 93144116760520780);
#[linkme::distributed_slice(PROPERTIES)]
static T_PROP: Property = Property {
    id: T,
    ty: Type::Number(NumericType::Float),
};

pub const INV_PROJECTION: Identifier = Identifier::new("inv_proj", 1835117139336577900);
#[linkme::distributed_slice(PROPERTIES)]
static INV_PROJECTION_PROP: Property = Property {
    id: INV_PROJECTION,
    ty: Type::Struct(MATRIX4),
};

pub const STEP_SIZE: Identifier = Identifier::new("step_size", 7777887281564637643);
#[linkme::distributed_slice(PROPERTIES)]
static STEP_SIZE_PROP: Property = Property {
    id: STEP_SIZE,
    ty: Type::Number(NumericType::UInt),
};

pub const EPSILON: Identifier = Identifier::new("epsilon", 32338215630771851);
#[linkme::distributed_slice(PROPERTIES)]
static EPSILON_PROP: Property = Property {
    id: EPSILON,
    ty: Type::Number(NumericType::Float),
};

pub const FRAC_1_K: Identifier = Identifier::new("frac_1_k", 5512322721559903899);
#[linkme::distributed_slice(PROPERTIES)]
static FRAC_1_K_PROP: Property = Property {
    id: FRAC_1_K,
    ty: Type::Number(NumericType::Float),
};

pub const CANDIDATE: Identifier = Identifier::new("candidate", 1956157168917067266);
#[linkme::distributed_slice(PROPERTIES)]
static CANDIDATE_PROP: Property = Property {
    id: CANDIDATE,
    ty: Type::Struct(CONTEXT),
};

pub const STEPS: Identifier = Identifier::new("steps", 1682585060223888912);
#[linkme::distributed_slice(PROPERTIES)]
static STEPS_PROP: Property = Property {
    id: STEPS,
    ty: Type::Number(NumericType::UInt),
};

pub const MAX_STEPS: Identifier = Identifier::new("max_steps", 1146747975614382616);
#[linkme::distributed_slice(PROPERTIES)]
static MAX_STEPS_PROP: Property = Property {
    id: MAX_STEPS,
    ty: Type::Number(NumericType::UInt),
};

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

pub struct Raymarch {
    pub max_steps: elysian_core::ast::expr::Expr,
    pub march: March,
    pub inv_projection: elysian_core::ast::expr::Expr,
    pub field: Box<dyn AsModule>,
}

impl Debug for Raymarch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Raymarch")
            .field("field", &self.field)
            .finish()
    }
}

impl Hash for Raymarch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl AsModule for Raymarch {
    fn entry_point(&self) -> Identifier {
        RAYMARCH
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &indexmap::IndexMap<Identifier, Type>,
        _: &Identifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        if !spec.contains(&POSITION_2D) {
            panic!("Raymarch is only compatible with the 2D Position domain");
        }

        if !spec.contains(&DISTANCE) {
            panic!("Raymarch requires the Distance domain");
        }

        let spec_3d = SpecializationData::new_3d();
        let field_entry_point = self.field.entry_point();

        let mut block = vec![];
        block.extend([
            MAX_STEPS.bind(self.max_steps.clone().into()),
            STEPS.bind(0u32.literal()),
        ]);

        block.extend([
            INV_PROJECTION.bind(self.inv_projection.clone().into()),
            RAY_FROM_4.bind(
                INV_PROJECTION.read()
                    * Expr::Struct(
                        VECTOR4,
                        [
                            (X, [CONTEXT, POSITION_2D, X].read()),
                            (Y, [CONTEXT, POSITION_2D, Y].read()),
                            (Z, 0.0.literal()),
                            (W, 1.0.literal()),
                        ]
                        .into_iter()
                        .collect(),
                    ),
            ),
            RAY_FROM_3.bind(Expr::Struct(
                VECTOR3,
                [
                    (X, [RAY_FROM_4, X].read() / [RAY_FROM_4, W].read()),
                    (Y, [RAY_FROM_4, Y].read() / [RAY_FROM_4, W].read()),
                    (Z, [RAY_FROM_4, Z].read() / [RAY_FROM_4, W].read()),
                ]
                .into_iter()
                .collect(),
            )),
            RAY_TO_4.bind(
                INV_PROJECTION.read()
                    * Expr::Struct(
                        VECTOR4,
                        [
                            (X, [CONTEXT, POSITION_2D, X].read()),
                            (Y, [CONTEXT, POSITION_2D, Y].read()),
                            (Z, -1.0.literal()),
                            (W, 1.0.literal()),
                        ]
                        .into_iter()
                        .collect(),
                    ),
            ),
            RAY_TO_3.bind(Expr::Struct(
                VECTOR3,
                [
                    (X, [RAY_TO_4, X].read() / [RAY_TO_4, W].read()),
                    (Y, [RAY_TO_4, Y].read() / [RAY_TO_4, W].read()),
                    (Z, [RAY_TO_4, Z].read() / [RAY_TO_4, W].read()),
                ]
                .into_iter()
                .collect(),
            )),
            RAY_DIR.bind((RAY_FROM_3.read() - RAY_TO_3.read()).normalize()),
            [CONTEXT, DISTANCE].write(Number::from(f32::MAX).literal()),
            T.bind(0.0.literal()),
        ]);

        let mut loop_body = vec![
            RAY_POS.bind(RAY_FROM_3.read() + RAY_DIR.read() * T.read()),
            [CONTEXT, POSITION_3D].write(Expr::Struct(
                VECTOR3,
                [
                    (X, [RAY_POS, X].read()),
                    (Y, [RAY_POS, Y].read()),
                    (Z, [RAY_POS, Z].read()),
                ]
                .into_iter()
                .collect(),
            )),
            CANDIDATE.bind(field_entry_point.call(CONTEXT.read())),
            CONTEXT.write(CANDIDATE.read()).if_else(
                [CANDIDATE, DISTANCE].read().lt([CONTEXT, DISTANCE].read()),
                None,
            ),
            Stmt::Break.if_else([CONTEXT, DISTANCE].read().lt(0.0.literal()), None),
            STEPS.write(STEPS.read() + 1u32.literal()),
            Stmt::Break.if_else(STEPS.read().gt(MAX_STEPS.read()), None),
        ];

        loop_body.push(match &self.march {
            March::Fixed { step_size } => T.write(T.read() + step_size.clone().into()),
            March::Sphere { epsilon } => T.write(
                T.read() + Expr::from(epsilon.clone()).max([CONTEXT, DISTANCE].read().abs()),
            ),
            March::Lipschitz { epsilon, falloff_k } => T.write(
                T.read()
                    + Expr::from(epsilon.clone())
                        .max([CONTEXT, DISTANCE].read().abs() * falloff_k.clone().into()),
            ),
        });

        block.extend([
            Stmt::Loop {
                stmt: Box::new(Stmt::Block(loop_body.block())),
            },
            CONTEXT.read().output(),
        ]);

        self.field
            .functions(&spec_3d, tys, &field_entry_point)
            .into_iter()
            .chain([FunctionDefinition {
                id: RAYMARCH,
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT,
                    mutable: true,
                }],
                output: CONTEXT,
                block: block.block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
