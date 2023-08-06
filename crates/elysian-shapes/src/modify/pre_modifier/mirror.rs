use std::{fmt::Debug, hash::Hash};

use elysian_core::{
    ast::expr::Expr,
    ir::{
        ast::{Block, GRADIENT_2D, GRADIENT_3D, POSITION_2D, POSITION_3D},
        module::{
            AsIR, DomainsDyn, DynAsIR, FunctionDefinition, FunctionIdentifier, InputDefinition,
            PropertyIdentifier, SpecializationData, CONTEXT,
        },
    },
};

use elysian_proc_macros::elysian_stmt;

pub const BASIS_MIRROR: FunctionIdentifier =
    FunctionIdentifier::new("basis_mirror", 2763069141557531361);

#[derive(Debug, Clone)]
pub enum MirrorMode {
    Basis(Expr),
    Axis(Expr),
}

impl ToString for MirrorMode {
    fn to_string(&self) -> String {
        match self {
            MirrorMode::Basis(_) => "basis",
            MirrorMode::Axis(_) => "axis",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Mirror {
    field: DynAsIR,
    mode: MirrorMode,
}

impl Hash for Mirror {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        BASIS_MIRROR.uuid().hash(state);
        state.write_u64(self.field.hash_ir());
    }
}

impl DomainsDyn for Mirror {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        vec![
            POSITION_2D.into(),
            POSITION_3D.into(),
            GRADIENT_2D.into(),
            GRADIENT_3D.into(),
        ]
        .into_iter()
        .chain(self.field.domains_dyn())
        .collect()
    }
}

impl AsIR for Mirror {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("basis_mirror".into()).specialize(spec)
    }

    fn functions(
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

        let field_call = field_entry_point.call(self.field.arguments(elysian_stmt! { CONTEXT }));

        block.push(elysian_stmt! {
            let position = CONTEXT.position
        });

        match &self.mode {
            MirrorMode::Basis(basis) => {
                let basis = elysian_core::ir::ast::Expr::from(basis.clone());
                block.push(elysian_stmt! {
                    CONTEXT.position = CONTEXT.position * (CONTEXT.position.sign() + (1.0 - #basis)).sign()
                });
            }
            MirrorMode::Axis(axis) => {
                let axis = elysian_core::ir::ast::Expr::from(axis.clone());
                block.push(elysian_stmt! {
                    CONTEXT.position = CONTEXT.position * CONTEXT.position.dot(#axis).sign()
                });
            }
        }

        block.push(elysian_stmt! {
            let CONTEXT = #field_call
        });

        if let Some(gradient) = gradient {
            match &self.mode {
                MirrorMode::Basis(basis) => {
                    let basis = elysian_core::ir::ast::Expr::from(basis.clone());
                    block.push(elysian_stmt! {
                        CONTEXT.gradient = CONTEXT.gradient * (position.sign() + (1.0 - #basis)).sign()
                    });
                }
                MirrorMode::Axis(axis) => {
                    let axis = elysian_core::ir::ast::Expr::from(axis.clone());
                    block.push(elysian_stmt! {
                        CONTEXT.gradient = CONTEXT.gradient * position.dot(#axis).sign()
                    });
                }
            }
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        self.field
            .functions(spec, &field_entry_point)
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

pub trait IntoMirror {
    fn mirror(self, mode: MirrorMode) -> Mirror;
}

impl<T> IntoMirror for T
where
    T: 'static + AsIR,
{
    fn mirror(self, mode: MirrorMode) -> Mirror {
        Mirror {
            field: Box::new(self),
            mode,
        }
    }
}
