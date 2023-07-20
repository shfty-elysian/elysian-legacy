use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use rust_gpu_bridge::{MaxBound, One, Two, Zero};

use crate::ir::{
    ast::{
        Expr, Identifier, IntoBind, IntoBlock, IntoLiteral, IntoRead, IntoValue, IntoWrite,
        Property, Stmt, TypeSpec, CONTEXT, DISTANCE, POSITION_2D, POSITION_3D, X, Y,
    },
    module::{AsModule, FunctionDefinition, InputDefinition, SpecializationData, Type},
};

use super::modify::CONTEXT_STRUCT;

pub const CROSS_SECTION: Identifier = Identifier::new("raymarch", 11670715461129592823);

pub const RAY_POS: Property = Property::new("ray_pos", Type::Vector3, 1031119209943889737);
pub const RAY_DIR: Property = Property::new("ray_dir", Type::Vector3, 1835117139336577900);
pub const STEP_SIZE: Property = Property::new("step_size", Type::Number, 7777887281564637643);
pub const TEMP: Property = Property::new("temp", Type::Struct(CONTEXT_STRUCT), 1956157168917067266);
pub const STEPS: Property = Property::new("steps", Type::Number, 1682585060223888912);
pub const MAX_STEPS: Property = Property::new("max_steps", Type::Number, 1146747975614382616);

pub struct Raymarch<T>
where
    T: TypeSpec,
{
    pub step_size: crate::ast::expr::Expr<T>,
    pub max_steps: crate::ast::expr::Expr<T>,
    pub field: Box<dyn AsModule<T>>,
}

impl<T> Debug for Raymarch<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Raymarch")
            .field("field", &self.field)
            .finish()
    }
}

impl<T> Hash for Raymarch<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir())
    }
}

impl<T> AsModule<T> for Raymarch<T>
where
    T: TypeSpec,
    T::NUMBER: Zero + One + Two + MaxBound + IntoValue<T>,
{
    fn entry_point(&self) -> Identifier {
        CROSS_SECTION
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        _: &Identifier,
    ) -> Vec<crate::ir::module::FunctionDefinition<T>> {
        if !spec.contains(POSITION_2D.id()) {
            panic!("CrossSection is only compatible with the 2D position domain");
        }

        let spec_3d = SpecializationData::new_3d();
        let field_entry_point = self.field.entry_point();
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
                block: [
                    RAY_POS.bind(Expr::vector3(
                        [CONTEXT, POSITION_2D, X].read(),
                        [CONTEXT, POSITION_2D, Y].read(),
                        T::NUMBER::ZERO.literal(),
                    )),
                    RAY_DIR.bind(Expr::vector3(
                        T::NUMBER::ZERO.literal(),
                        T::NUMBER::ZERO.literal(),
                        -T::NUMBER::ONE.literal(),
                    )),
                    STEP_SIZE.bind(self.step_size.clone().into()),
                    STEPS.bind(T::NUMBER::ZERO.literal()),
                    MAX_STEPS.bind(self.max_steps.clone().into()),
                    [CONTEXT, DISTANCE].write(STEP_SIZE.read() * MAX_STEPS.read()),
                    Stmt::Loop {
                        stmt: Box::new(Stmt::Block(
                            [
                                [CONTEXT, POSITION_3D].bind(RAY_POS.read()),
                                TEMP.bind(field_entry_point.call(CONTEXT.read())),
                                CONTEXT.write(TEMP.read()).if_else(
                                    [TEMP, DISTANCE].read().lt([CONTEXT, DISTANCE].read()),
                                    None,
                                ),
                                Stmt::Break.if_else(
                                    [CONTEXT, DISTANCE].read().lt(T::NUMBER::ZERO.literal()),
                                    None,
                                ),
                                STEPS.write(STEPS.read() + T::NUMBER::ONE.literal()),
                                Stmt::Break.if_else(STEPS.read().gt(MAX_STEPS.read()), None),
                                RAY_POS.write(RAY_POS.read() + RAY_DIR.read() * STEP_SIZE.read()),
                            ]
                            .block(),
                        )),
                    },
                    CONTEXT.read().output(),
                ]
                .block(),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<crate::ir::module::StructDefinition> {
        self.field.structs()
    }
}
