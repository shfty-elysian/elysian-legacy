use elysian_core::ast::{
    alias::{Capsule, Circle, Ring},
    attribute::Attribute::{self, *},
    combinator::{
        Blend, Boolean,
        Combinator::{self, *},
    },
    expand::Expand,
    expr::IntoLiteral,
    field::Field,
    Elysian, IntoAlias, IntoCombine,
};

pub fn kettle_bell() -> Elysian<f32, [f32; 2]> {
    let smooth_union = [
        Combinator::Boolean(Boolean::Union),
        Blend(Blend::SmoothUnion {
            attr: Distance,
            k: 0.4.literal(),
        }),
        Blend(Blend::SmoothUnion {
            attr: Gradient,
            k: 0.4.literal(),
        }),
    ];

    let smooth_subtraction = [
        Combinator::Boolean(Boolean::Subtraction),
        Combinator::Blend(Blend::SmoothSubtraction {
            attr: Attribute::Distance,
            k: 0.4.literal(),
        }),
        Combinator::Blend(Blend::SmoothSubtraction {
            attr: Attribute::Gradient,
            k: 0.4.literal(),
        }),
    ];

    [
        [
            Circle {
                radius: 1.0.literal(),
            }
            .alias()
            .expand()
            .translate([0.0, 0.5].literal()),
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .alias()
            .expand()
            .translate([0.0, -0.25].literal()),
        ]
        .combine(smooth_union),
        Capsule {
            dir: [1.5, 0.0].literal(),
            radius: 0.2.literal(),
        }
        .alias()
        .expand()
        .translate([0.0, 0.5].literal()),
    ]
    .combine(smooth_subtraction)
}

pub fn point() -> Elysian<f32, [f32; 2]> {
    Elysian::Field {
        pre_modifiers: Default::default(),
        field: Field::Point,
        post_modifiers: Default::default(),
    }
}

pub fn shapes() -> [(&'static str, Elysian<f32, [f32; 2]>); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
