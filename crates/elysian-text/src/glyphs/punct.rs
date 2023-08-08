use elysian_core::{ast::combine::Combine, ir::module::IntoAsIR};
use elysian_shapes::{
    combine::Union,
    field::{Line, Point, Infinity},
    modify::IntoTranslate,
};

pub fn space() -> impl IntoAsIR {
    Infinity
}

pub fn period() -> impl IntoAsIR {
    Point.translate([0.0, -1.0])
}

pub fn comma() -> impl IntoAsIR {
    Line::segment([0.2, 0.4]).translate([0.0, -1.0])
}

pub fn exclamation() -> impl IntoAsIR {
    Combine::from(Union)
        .push(Point.translate([0.0, -1.0]))
        .push(Line::segment([0.0, 1.0]))
}
