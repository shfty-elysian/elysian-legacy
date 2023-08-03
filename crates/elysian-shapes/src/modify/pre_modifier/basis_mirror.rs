use std::{fmt::Debug, hash::Hash};

use elysian_core::ir::{
    ast::{Block, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D},
    module::{
        AsIR, Domains, FunctionDefinition, FunctionIdentifier, InputDefinition,
        PropertyIdentifier, SpecializationData, CONTEXT,
    },
};

use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const BASIS_MIRROR: FunctionIdentifier =
    FunctionIdentifier::new("basis_mirror", 2763069141557531361);

#[derive(Debug)]
pub struct BasisMirror {
    field: Box<dyn AsIR>,
}

impl Hash for BasisMirror {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        BASIS_MIRROR.uuid().hash(state);
        state.write_u64(self.field.hash_ir());
    }
}

impl Domains for BasisMirror {
    fn domains() -> Vec<PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            GRADIENT_2D.into(),
            GRADIENT_3D.into(),
        ]
    }
}

impl AsIR for BasisMirror {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("basis_mirror")
    }

    fn functions_impl(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let field_entry_point = self.field.entry_point(spec);

        let position = if spec.contains(&POSITION_2D.into()) {
            POSITION_2D
        } else if spec.contains(&POSITION_3D.into()) {
            POSITION_3D
        } else {
            panic!("No position domain")
        };

        let gradient = if spec.contains(&GRADIENT_2D.into()) {
            Some(GRADIENT_2D)
        } else if spec.contains(&GRADIENT_3D.into()) {
            Some(GRADIENT_3D)
        } else {
            None
        };

        let mut block = Block::default();

        block.extend(elysian_block! {
            let position = CONTEXT.position;
            CONTEXT.position = CONTEXT.position.abs();
            let CONTEXT = field_entry_point(CONTEXT);
        });

        if let Some(gradient) = gradient {
            block.push(elysian_stmt! {
                CONTEXT.gradient = CONTEXT.gradient * position.sign()
            });
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        self.field
            .functions_impl(spec, &field_entry_point)
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                }],
                output: CONTEXT.into(),
                block,
            })
            .collect()
    }

    fn structs(&self) -> Vec<elysian_core::ir::module::StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoBasisMirror {
    fn basis_mirror(self) -> BasisMirror;
}

impl<T> IntoBasisMirror for T
where
    T: 'static + AsIR,
{
    fn basis_mirror(self) -> BasisMirror {
        BasisMirror {
            field: Box::new(self),
        }
    }
}
