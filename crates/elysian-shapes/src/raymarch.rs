use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        ast::{
            Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, Number, Property, Stmt,
            CONTEXT, DISTANCE, MATRIX4_STRUCT, POSITION_2D, POSITION_3D, VECTOR3_STRUCT,
            VECTOR4_STRUCT, W, X, Y, Z,
        },
        module::{
            AsModule, FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type,
        },
    },
};

pub const RAYMARCH: Identifier = Identifier::new("raymarch", 11670715461129592823);

pub const RAY_FROM_4: Property = Property::new(
    "ray_from_4",
    Type::Struct(VECTOR4_STRUCT),
    1031119209943889737,
);
pub const RAY_TO_4: Property = Property::new(
    "ray_to_4",
    Type::Struct(VECTOR4_STRUCT),
    1362247063737049192,
);

pub const RAY_FROM_3: Property = Property::new(
    "ray_from_3",
    Type::Struct(VECTOR3_STRUCT),
    1031119209943889737,
);
pub const RAY_TO_3: Property = Property::new(
    "ray_to_3",
    Type::Struct(VECTOR3_STRUCT),
    1362247063737049192,
);

pub const RAY_POS: Property =
    Property::new("ray_pos", Type::Struct(VECTOR3_STRUCT), 203470946369255426);
pub const RAY_DIR: Property = Property::new(
    "ray_dir",
    Type::Struct(VECTOR3_STRUCT),
    11883607992066663879,
);
pub const T: Property = Property::new("t", Type::Number(NumericType::Float), 93144116760520780);
pub const INV_PROJECTION: Property = Property::new(
    "inv_proj",
    Type::Struct(MATRIX4_STRUCT),
    1835117139336577900,
);
pub const STEP_SIZE: Property = Property::new(
    "step_size",
    Type::Number(NumericType::Float),
    7777887281564637643,
);
pub const EPSILON: Property = Property::new(
    "epsilon",
    Type::Number(NumericType::Float),
    32338215630771851,
);
pub const FRAC_1_K: Property = Property::new(
    "frac_1_k",
    Type::Number(NumericType::Float),
    5512322721559903899,
);
pub const CANDIDATE: Property = Property::new(
    "candidate",
    Type::Struct(CONTEXT_STRUCT),
    1956157168917067266,
);
pub const STEPS: Property = Property::new(
    "steps",
    Type::Number(NumericType::UInt),
    1682585060223888912,
);
pub const MAX_STEPS: Property = Property::new(
    "max_steps",
    Type::Number(NumericType::UInt),
    1146747975614382616,
);

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
        _: &Identifier,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        if !spec.contains(POSITION_2D.id()) {
            panic!("Raymarch is only compatible with the 2D Position domain");
        }

        if !spec.contains(DISTANCE.id()) {
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
                        VECTOR4_STRUCT,
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
                VECTOR3_STRUCT,
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
                        VECTOR4_STRUCT,
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
                VECTOR3_STRUCT,
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
                VECTOR3_STRUCT,
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
            .functions(&spec_3d, &field_entry_point)
            .into_iter()
            .chain([FunctionDefinition {
                id: RAYMARCH,
                public: false,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                }],
                output: CONTEXT_STRUCT.clone(),
                block: block.block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
