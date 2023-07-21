use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_core::{ir::{
    ast::{
        Expr, Identifier, IntoBlock, IntoLiteral, IntoRead, IntoWrite, Number, Property, Stmt,
        CONTEXT, DISTANCE, POSITION_2D, POSITION_3D, W, X, Y, Z,
    },
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData, Type},
}, ast::modify::CONTEXT_STRUCT};

pub const CROSS_SECTION: Identifier = Identifier::new("raymarch", 11670715461129592823);

pub const RAY_FROM: Property = Property::new("ray_from", Type::Vector3, 1031119209943889737);
pub const RAY_TO: Property = Property::new("ray_to", Type::Vector3, 1362247063737049192);
pub const RAY_POS: Property = Property::new("ray_pos", Type::Vector3, 203470946369255426);
pub const RAY_DIR: Property = Property::new("ray_dir", Type::Vector3, 11883607992066663879);
pub const T: Property = Property::new("t", Type::Number, 93144116760520780);
pub const INV_PROJECTION: Property = Property::new("inv_proj", Type::Matrix4, 1835117139336577900);
pub const STEP_SIZE: Property = Property::new("step_size", Type::Number, 7777887281564637643);
pub const EPSILON: Property = Property::new("epsilon", Type::Number, 32338215630771851);
pub const FRAC_1_K: Property = Property::new("frac_1_k", Type::Number, 5512322721559903899);
pub const CANDIDATE: Property = Property::new(
    "candidate",
    Type::Struct(CONTEXT_STRUCT),
    1956157168917067266,
);
pub const STEPS: Property = Property::new("steps", Type::Number, 1682585060223888912);
pub const MAX_STEPS: Property = Property::new("max_steps", Type::Number, 1146747975614382616);

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
        CROSS_SECTION
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
            RAY_FROM.bind(
                INV_PROJECTION.read()
                    * Expr::vector4(
                        [CONTEXT, POSITION_2D, X].read(),
                        [CONTEXT, POSITION_2D, Y].read(),
                        0.0_f32.literal(),
                        1.0_f32.literal(),
                    ),
            ),
            RAY_FROM.write(RAY_FROM.read() / [RAY_FROM, W].read()),
            RAY_TO.bind(
                INV_PROJECTION.read()
                    * Expr::vector4(
                        [CONTEXT, POSITION_2D, X].read(),
                        [CONTEXT, POSITION_2D, Y].read(),
                        -1.0_f32.literal(),
                        1.0_f32.literal(),
                    ),
            ),
            RAY_TO.write(RAY_TO.read() / [RAY_TO, W].read()),
            RAY_DIR.bind((RAY_FROM.read() - RAY_TO.read()).normalize()),
            [CONTEXT, DISTANCE].write(Number::from(f32::MAX).literal()),
            T.bind(0.0_f32.literal()),
        ]);

        let mut loop_body = vec![
            RAY_POS.bind(RAY_FROM.read() + RAY_DIR.read() * T.read()),
            [CONTEXT, POSITION_3D].write(Expr::vector3(
                [RAY_POS, X].read(),
                [RAY_POS, Y].read(),
                [RAY_POS, Z].read(),
            )),
            CANDIDATE.bind(field_entry_point.call(CONTEXT.read())),
            CONTEXT.write(CANDIDATE.read()).if_else(
                [CANDIDATE, DISTANCE].read().lt([CONTEXT, DISTANCE].read()),
                None,
            ),
            Stmt::Break.if_else(
                [CONTEXT, DISTANCE].read().lt(0.0_f32.literal()),
                None,
            ),
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
                id: CROSS_SECTION,
                public: false,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                }],
                output: CONTEXT_STRUCT,
                block: block.block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}
