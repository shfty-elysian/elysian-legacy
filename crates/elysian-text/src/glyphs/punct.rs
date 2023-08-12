use elysian_shapes::{
    combine::{Combine, Union},
    field::{Infinity, Line, Point},
    modify::IntoTranslate,
    shape::IntoShape,
};

pub fn space() -> impl IntoShape {
    Infinity
}

pub fn period(cell_size: [f64; 2]) -> impl IntoShape {
    Point.translate([0.0, -cell_size[1]])
}

pub fn comma(cell_size: [f64; 2]) -> impl IntoShape {
    Line::segment([cell_size[0] * 0.2, cell_size[1] * 0.4])
        .translate([-cell_size[0] * 0.1, -cell_size[1]])
}

pub fn exclamation(cell_size: [f64; 2]) -> impl IntoShape {
    Combine::from(Union)
        .push(period(cell_size))
        .push(Line::segment([0.0, cell_size[1]]))
}
