use sandbox_lib::{
    elysian::{
        alias::{Capsule, Circle, Ring},
        attribute::Attribute::*,
        combinator::Blend::*,
        combinator::Boolean::*,
        combinator::Combinator::*,
        expr::IntoLiteral,
        Field::*,
        IntoAlias, IntoCombine,
    },
    frontend::out_image::rasterize,
};

use viuer::Config;

fn main() {
    let smooth_union = [
        Boolean(Union),
        /*
        Blend(SmoothUnion {
            attr: Distance,
            k: 0.4.literal(),
        }),
        Blend(SmoothUnion {
            attr: Gradient,
            k: 0.4.literal(),
        }),
        */
    ];

    let smooth_subtraction = [
        Boolean(Subtraction),
        Blend(SmoothSubtraction {
            attr: Distance,
            k: 0.4.literal(),
        }),
        Blend(SmoothSubtraction {
            attr: Gradient,
            k: 0.4.literal(),
        }),
    ];

    let shape = [
        [
            Circle {
                radius: 1.0.literal(),
            }
            .alias()
            .translate([0.0, 0.5].literal()),
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .alias()
            .translate([0.0, -0.25].literal()),
        ]
        .combine(smooth_union),
        Capsule {
            dir: [1.5, 0.0].literal(),
            radius: 0.2.literal(),
        }
        .alias()
        .translate([0.0, 0.5].literal()),
    ]
    .combine(smooth_subtraction);

    let shape = Point
        .field()
        .isosurface(1.0.literal())
        .translate([0.0, -1.0].literal());

    println!("{shape:#?}");

    let image = rasterize(&shape, 48, 48, 0.5);
    viuer::print(
        &image.into(),
        &Config {
            transparent: false,
            absolute_offset: false,
            x: 0,
            y: 0,
            restore_cursor: false,
            width: None,
            height: None,
            truecolor: true,
            use_kitty: true,
            use_iterm: true,
        },
    )
    .unwrap();
}
