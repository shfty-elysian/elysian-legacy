use elysian_core::ir::module::IntoAsIR;
use elysian_shapes::{
    field::Line,
    modify::{IntoMirror, IntoTranslate},
};

use super::combinator;

pub fn a() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.5, 0.0]).translate([0.0, -0.5]))
        .push(Line::segment([1.0, -2.25]).translate([0.0, 1.0]))
        .mirror_basis([1.0, 0.0])
        .as_ir()
}

pub fn b() -> () {
    todo!()
}

pub fn c() -> () {
    todo!()
}

pub fn d() -> () {
    todo!()
}

pub fn e() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.0, 0.8]).translate([-1.0, 0.0]))
        .push(Line::segment([0.8, 0.0]).translate([-0.8, 0.0]))
        .push(Line::centered([0.8, 0.0]).translate([0.0, 1.0]))
        .mirror_basis([0.0, 1.0])
}

pub fn f() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([0.0, 1.0]).translate([-1.0, -0.2]))
        .push(Line::segment([0.8, 0.0]).translate([-0.8, 0.0]))
        .push(Line::centered([0.8, 0.0]).translate([0.0, 1.0]))
}

pub fn g() -> () {
    todo!()
}

pub fn h() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([0.0, 1.0]).translate([1.0, 0.0]))
        .push(Line::segment([0.8, 0.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn i() -> impl IntoAsIR {
    Line::centered([0.0, 1.0])
}

pub fn j() -> () {
    todo!()
}

pub fn k() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(Line::segment([1.6, 1.0]).translate([-0.8, 0.0]))
        .mirror_basis([0.0, 1.0])
}

pub fn l() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(Line::centered([0.8, 0.0]).translate([0.0, -1.2]))
}

pub fn m() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.0, 2.0]).translate([1.0, -1.0]))
        .push(Line::segment([1.0, 1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn n() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(
            Line::segment([0.0, 1.0])
                .translate([1.0, 0.0])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered([-0.8, 1.0]))
}

pub fn o() -> () {
    todo!()
}

pub fn p() -> () {
    todo!()
}

pub fn q() -> () {
    todo!()
}

pub fn r() -> () {
    todo!()
}

pub fn s() -> () {
    todo!()
}

pub fn t() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([1.0, 0.0]))
        .push(Line::centered([0.0, 1.0]).translate([0.0, -1.2]))
        .translate([0.0, 1.0])
}

pub fn u() -> () {
    todo!()
}

pub fn v() -> impl IntoAsIR {
    Line::segment([1.0, 2.0])
        .translate([0.0, -1.0])
        .mirror_basis([1.0, 0.0])
}

pub fn w() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.5, 2.0]).translate([0.5, -1.0]))
        .push(Line::segment([0.5, -1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn x() -> impl IntoAsIR {
    Line::segment([0.6, 1.0]).mirror_basis([1.0, 1.0])
}

pub fn y() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([1.0, 1.0]))
        .push(Line::segment([0.0, -1.2]))
        .mirror_basis([1.0, 0.0])
}

pub fn z() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(
            Line::segment([1.0, 0.0])
                .translate([0.0, 1.2])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered([1.0, 1.0]))
}
