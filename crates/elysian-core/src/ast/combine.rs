use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

use crate::ir::ast::{COMBINE_CONTEXT, COMBINE_CONTEXT_PROP};
use crate::ir::module::{IntoRead, PropertyIdentifier, Type, CONTEXT_PROP};
use crate::ir::{
    as_ir::{AsIR, DynAsIR, HashIR},
    ast::{Block, Expr, Identifier},
    module::{
        AsModule, DynAsModule, FieldDefinition, FunctionDefinition, InputDefinition,
        SpecializationData, StructDefinition, CONTEXT,
    },
};
use crate::property;

pub const LEFT: PropertyIdentifier = PropertyIdentifier::new("left", 635254731934742132);
property!(LEFT, LEFT_PROP_DEF, Type::Struct(CONTEXT));

pub const RIGHT: PropertyIdentifier = PropertyIdentifier::new("right", 5251097991491214179);
property!(RIGHT, RIGHT_PROP_DEF, Type::Struct(CONTEXT));

pub const OUT: PropertyIdentifier = PropertyIdentifier::new("out", 1470763158891875334);
property!(OUT, OUT_PROP_DEF, Type::Struct(CONTEXT));

pub const COMBINE_CONTEXT_STRUCT_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: LEFT,
        public: false,
    },
    FieldDefinition {
        id: RIGHT,
        public: false,
    },
    FieldDefinition {
        id: OUT,
        public: false,
    },
];

pub const COMBINE_CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
    id: COMBINE_CONTEXT,
    public: false,
    fields: Cow::Borrowed(COMBINE_CONTEXT_STRUCT_FIELDS),
};

pub struct Combine {
    pub combinator: Vec<Box<dyn AsIR>>,
    pub shapes: Vec<DynAsModule>,
}

impl Debug for Combine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Combine")
            .field("combinator", &self.combinator)
            .field("shapes", &self.shapes)
            .finish()
    }
}

impl Hash for Combine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for combinator in &self.combinator {
            state.write_u64(combinator.hash_ir())
        }
        for shape in &self.shapes {
            state.write_u64(shape.hash_ir())
        }
    }
}

impl AsModule for Combine {
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("combine")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<PropertyIdentifier, Type>,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        let (shape_entry_points, shape_functions): (Vec<_>, Vec<_>) = self
            .shapes
            .iter()
            .map(|shape| {
                let entry_point = shape.entry_point();
                (
                    entry_point.clone(),
                    shape.functions(spec, tys, &entry_point),
                )
            })
            .unzip();

        let shape_functions: Vec<_> = shape_functions.into_iter().flatten().collect();

        let mut iter = shape_entry_points.iter();
        let base = iter.next().expect("Empty list").clone();

        let mut block = vec![];

        iter.fold(base.call(CONTEXT_PROP.read()), |acc, next| {
            block.push(COMBINE_CONTEXT_PROP.bind(
                COMBINE_CONTEXT.construct([(LEFT, acc), (RIGHT, next.call(CONTEXT_PROP.read()))]),
            ));
            block.push(COMBINE_CONTEXT_PROP.bind(self.combinator.iter().fold(
                COMBINE_CONTEXT_PROP.read(),
                |acc: Expr, next| {
                    let Expr::Call{ function, args } = next.expression(spec, acc) else  {
                            panic!("Combinator expression is not a Call")
                        };

                    Expr::Call { function, args }
                },
            )));
            block.push(OUT.bind([COMBINE_CONTEXT_PROP, OUT].read()));
            OUT.read()
        });

        block.push(OUT.read().output());

        let block = Block(block);

        self.combinator
            .iter()
            .flat_map(|t| t.functions(spec))
            .chain(shape_functions)
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: CONTEXT_PROP,
                    mutable: false,
                }],
                output: CONTEXT_PROP,
                block,
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![COMBINE_CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoCombine {
    fn combine<U>(self, combinator: U) -> Combine
    where
        U: IntoIterator<Item = DynAsIR>;
}

impl<T> IntoCombine for T
where
    T: IntoIterator<Item = DynAsModule>,
{
    fn combine<V>(self, combinator: V) -> Combine
    where
        V: IntoIterator<Item = DynAsIR>,
    {
        let shapes: Vec<_> = self.into_iter().collect();
        assert!(shapes.len() >= 2, "Combine must have at least two shapes");
        Combine {
            combinator: combinator.into_iter().collect(),
            shapes,
        }
    }
}
