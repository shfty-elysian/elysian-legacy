use elysian_core::{
    ast::{
        combine::{Combinator, Combine},
        expr::{Expr, IntoLiteral, IntoRead},
        filter::IntoFilter,
        modify::IntoModify,
        select::Select,
    },
    ir::{
        ast::{vector2, COLOR, DISTANCE, GRADIENT_2D, POSITION_3D, UV, X, Y},
        module::{DynAsIR, IntoAsIR},
    },
};
use elysian_shapes::{
    color::{
        ambient_light_color, directional_light_color, distance_color, normal_color,
        position_3d_color, uv_color,
    },
    combine::{SmoothSubtraction, SmoothUnion, Subtraction, Union},
    field::{Capsule, Chebyshev, Circle, Corner, Infinity, Line, Point, Quad, Ring},
    modify::{
        BoundType, IntoAspect, IntoDistanceBound, IntoGradientNormals, IntoIsosurface,
        IntoManifold, IntoRepeat, IntoSet, IntoTranslate, ASPECT, REPEAT_ID_2D,
    },
    raymarch::Raymarch,
    scale::IntoScale,
    uv_map::IntoUvMap,
    voronoi::{voronoi, CELL_ID},
};
use elysian_text::glyphs::{greek::sigma, text, Align};
use rust_gpu_bridge::glam::Mat4;

pub fn point() -> impl IntoAsIR {
    Point.gradient_normals().set_post(COLOR, normal_color())
}

pub fn chebyshev() -> impl IntoAsIR {
    Chebyshev.gradient_normals().set_post(COLOR, normal_color())
}

pub fn line() -> impl IntoAsIR {
    Line::centered([1.0, 0.0]).set_post(COLOR, uv_color())
}

pub fn circle() -> impl IntoAsIR {
    Circle::new(0.5)
        .gradient_normals()
        .set_post(COLOR, uv_color())
}

pub fn capsule() -> impl IntoAsIR {
    Capsule::new([1.5, 0.0], 0.5).set_post(COLOR, uv_color())
}

pub fn ring() -> impl IntoAsIR {
    Ring::new(1.0, 0.2).set_post(COLOR, uv_color())
}

pub fn union() -> impl IntoAsIR {
    Combine::from(Union).push(circle()).push(line())
}

pub fn smooth_union() -> impl IntoAsIR {
    Combinator::build()
        .push(Union)
        .push(SmoothUnion::new(DISTANCE, 0.4))
        .push(SmoothUnion::new(GRADIENT_2D, 0.4))
        .push(SmoothUnion::new(UV, 0.4))
        .combine()
        .push(circle())
        .push(line())
}

pub fn kettle_bell() -> impl IntoAsIR {
    Combinator::build()
        .push(Subtraction)
        .push(SmoothSubtraction::new(DISTANCE, 0.4))
        .push(SmoothSubtraction::new(GRADIENT_2D, 0.4))
        .push(SmoothSubtraction::new(UV, 0.4))
        .combine()
        .push(
            Combinator::build()
                .push(Union)
                .push(SmoothUnion::new(DISTANCE, 0.4))
                .push(SmoothUnion::new(GRADIENT_2D, 0.4))
                .push(SmoothUnion::new(UV, 0.4))
                .combine()
                .push(Circle::new(1.0).translate([0.0, -0.5]))
                .push(Ring::new(0.9, 0.15).translate([0.0, 0.25]))
                .push(Capsule::new([1.5, 0.0], 0.2).translate([0.0, -0.5])),
        )
        .push(Capsule::new([1.5, 0.0], 0.2).translate([0.0, -0.5]))
        .gradient_normals()
        .set_post(COLOR, uv_color())
}

pub fn select() -> impl IntoAsIR {
    let id_x = REPEAT_ID_2D.path().push(X).read();
    let id_x_lt = |t: f64| id_x.clone().lt(t);

    let id_y = REPEAT_ID_2D.path().push(Y).read();
    let id_y_lt = |t: f64| id_y.clone().lt(t);

    use elysian_text::glyphs::upper::*;

    Select::new(Infinity)
        .case(
            id_x_lt(1.0).and(id_y_lt(1.0)),
            a([1.0, 1.0]).set_post(COLOR, distance_color(1.0)),
        )
        .case(
            id_x_lt(2.0).and(id_y_lt(1.0)),
            n([1.0, 1.0]).set_post(COLOR, distance_color(1.0)),
        )
        .case(id_x_lt(3.0).and(id_y_lt(1.0)), point())
        .case(id_x_lt(4.0).and(id_y_lt(1.0)), chebyshev())
        .case(id_x_lt(5.0).and(id_y_lt(1.0)), raymarched())
        .case(
            id_x_lt(1.0).and(id_y_lt(2.0)),
            sigma().set_post(COLOR, distance_color(1.0)),
        )
        .case(
            id_x_lt(2.0).and(id_y_lt(2.0)),
            l([1.0, 1.0]).set_post(COLOR, distance_color(1.0)),
        )
        .case(
            id_x_lt(3.0).and(id_y_lt(2.0)),
            y([1.0, 1.0]).set_post(COLOR, distance_color(1.0)),
        )
        .case(
            id_x_lt(4.0).and(id_y_lt(2.0)),
            z([1.0, 1.0]).set_post(COLOR, distance_color(1.0)),
        )
        .case(
            id_x_lt(5.0).and(id_y_lt(2.0)),
            i([1.0, 1.0]).set_post(COLOR, distance_color(1.0)),
        )
        .scale(0.35)
        .aspect(ASPECT.prop().read())
        .repeat_clamped([0.8, 2.0], [0.0, 0.0], [4.0, 2.0])
        .translate([-1.6, -1.0])
}

pub fn raymarched() -> impl IntoAsIR {
    //let projection = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
    let projection = Mat4::perspective_infinite_rh(std::f32::consts::PI * 0.25, 1.0, 0.01);
    Raymarch::sphere(
        0.0001,
        100u64,
        projection.inverse(),
        Combine::from(Union)
            .push(Circle::new(1.5).translate([0.0, 1.0, -5.0]))
            .push(Capsule::new([0.5, 0.0, 0.0], 1.2).translate([-2.5, -1.5, -5.0]))
            .push(
                Quad::new([1.0, 1.0, 1.0], [GRADIENT_2D, UV])
                    .translate([2.5, -1.5, -5.0])
            ),
    )
}

pub fn partition() -> impl IntoAsIR {
    let cell_id = CELL_ID.prop().read();
    let cell_id_lt = |t: f64| cell_id.clone().lt(t);

    use elysian_text::glyphs::upper::*;

    Select::new(Infinity)
        .case(cell_id_lt(1.0), h([1.0, 1.0]))
        .case(cell_id_lt(2.0), e([1.0, 1.0]))
        .case(cell_id_lt(3.0), l([1.0, 1.0]))
        .case(cell_id_lt(4.0), l([1.0, 1.0]))
        .case(cell_id_lt(5.0), o([1.0, 1.0]))
        .case(cell_id_lt(6.0), w([1.0, 1.0]))
        .case(cell_id_lt(7.0), o([1.0, 1.0]))
        .case(cell_id_lt(8.0), r([1.0, 1.0]))
        .case(cell_id_lt(9.0), l([1.0, 1.0]))
        .case(cell_id_lt(10.0), d([1.0, 1.0]))
        .case(cell_id_lt(11.0), Infinity)
        .scale(0.35)
        .modify()
        .push_pre(voronoi([
            [-2.0, 0.5],
            [-1.0, 1.0],
            [0.0, 1.5],
            [1.0, 1.0],
            [2.0, 0.5],
            [-2.0, -0.5],
            [-1.0, -1.0],
            [0.0, -1.5],
            [1.0, -1.0],
            [2.0, -0.5],
            [0.0, 0.0],
        ]))
        .set_post(COLOR, distance_color(1.0))
        .aspect(ASPECT.prop().read())
}

pub fn pangram() -> impl IntoAsIR {
    text(
        "SPHINX OF\nBLACK QUARTZ,\nJUDGE MY VOW.",
        Align::Center,
        [0.4, 0.5],
        [1.0, 1.0],
        Some(
            |field: DynAsIR, cell_size: [f64; 2], total_size: [f64; 2]| {
                Combine::from(Union)
                    /*
                    .push(
                        Quad::new([cell_size[0], cell_size[1]], [GRADIENT_2D])
                            .manifold()
                            .set_post(COLOR, [1.0, 0.0, 1.0, 1.0].literal() * distance_color(25.0)),
                    )
                    .push(
                        Quad::new([total_size[0] * 0.5, total_size[1] * 0.5], [GRADIENT_2D])
                            .manifold()
                            .set_post(COLOR, [1.0, 1.0, 0.0, 1.0].literal() * distance_color(25.0)),
                    )
                    */
                    .push(field.isosurface(0.15).gradient_normals().set_post(
                        COLOR,
                        (ambient_light_color(0.25)
                            + directional_light_color([-1.0, -1.0, -1.0].literal().normalize()))
                            * distance_color(100.0),
                    ))
                    //.push(local_origin())
                    .as_ir()
            },
        ),
    )
}

pub fn test_shape() -> impl IntoAsIR {
    raymarched()
        .set_post(UV, UV.prop().read() * Expr::vector2(16.0, 16.0))
         .uv_map(pangram().filter(COLOR))
        //.set_post(COLOR, uv_color() * distance_color(5.0))
        //.set_post(COLOR, position_3d_color())
        .set_post(COLOR, COLOR.prop().read() * distance_color(100.0))
        //.set_post(COLOR, distance_color(100.0))
        .aspect(ASPECT.prop().read())
}

pub fn shapes() -> impl IntoIterator<Item = (&'static str, DynAsIR)> {
    [("test_shape", test_shape().as_ir())]
}
