use elysian_core::ir::module::IntoAsIR;
use elysian_shapes::{
    combine::{Combine, Union},
    field::{Infinity, Line, Point},
    modify::IntoTranslate,
};

pub fn space() -> impl IntoAsIR {
    Infinity
}

pub fn period(cell_size: [f64; 2]) -> impl IntoAsIR {
    Point.translate([0.0, -cell_size[1]])
}

pub fn comma(cell_size: [f64; 2]) -> impl IntoAsIR {
    Line::segment([cell_size[0] * 0.2, cell_size[1] * 0.4])
        .translate([-cell_size[0] * 0.1, -cell_size[1]])
}

pub fn exclamation(cell_size: [f64; 2]) -> impl IntoAsIR {
    Combine::from(Union)
        .push(period(cell_size))
        .push(Line::segment([0.0, cell_size[1]]))
}
