use elysian_shapes::{
    combine::{Combine, Union},
    field::{Infinity, Line, Point},
    modify::IntoTranslate,
    shape::IntoShape,
};

pub fn space() -> impl IntoShape {
    Infinity
}

pub fn period([_, height]: [f64; 2]) -> impl IntoShape {
    Point.translate([0.0, -height])
}

pub fn comma([width, height]: [f64; 2]) -> impl IntoShape {
    Line::segment([width * 0.2, height * 0.4]).translate([-width * 0.1, -height])
}

pub fn exclamation(cell_size @ [_, height]: [f64; 2]) -> impl IntoShape {
    Combine::from(Union)
        .push(period(cell_size))
        .push(Line::segment([0.0, height]))
}
