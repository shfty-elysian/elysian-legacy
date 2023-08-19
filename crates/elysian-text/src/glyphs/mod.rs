use elysian_core::expr::IntoPath;
use elysian_ir::ast::{X, Y};
use elysian_shapes::prepass::IntoPrepass;
use elysian_shapes::shape::DynShape;
use elysian_shapes::wrap::filter::IntoFilter;
use elysian_shapes::{combine::CombineBuilder, shape::IntoShape};
use elysian_shapes::{
    combine::Union,
    field::Infinity,
    modify::{IntoRepeat, IntoTranslate, REPEAT_ID_2D},
    select::Select,
};

pub mod greek;
pub mod lower;
pub mod punct;
pub mod upper;

pub fn combinator() -> CombineBuilder {
    CombineBuilder::build().push(Union)
}

pub fn char_field(char: char, cell_size: [f64; 2]) -> impl IntoShape {
    match char {
        'a' => lower::a().shape(),
        'b' => lower::b().shape(),
        'c' => lower::c().shape(),
        'd' => lower::d().shape(),
        'e' => lower::e().shape(),
        'f' => lower::f().shape(),
        'g' => lower::g().shape(),
        'h' => lower::h().shape(),
        'i' => lower::i().shape(),
        'j' => lower::j().shape(),
        'k' => lower::k().shape(),
        'l' => lower::l().shape(),
        'm' => lower::m().shape(),
        'n' => lower::n().shape(),
        'o' => lower::o().shape(),
        'p' => lower::p().shape(),
        'q' => lower::q().shape(),
        'r' => lower::r().shape(),
        's' => lower::s().shape(),
        't' => lower::t().shape(),
        'u' => lower::u().shape(),
        'v' => lower::v().shape(),
        'w' => lower::w().shape(),
        'x' => lower::x().shape(),
        'y' => lower::y().shape(),
        'z' => lower::z().shape(),
        'A' => upper::a(cell_size).shape(),
        'B' => upper::b(cell_size).shape(),
        'C' => upper::c(cell_size).shape(),
        'D' => upper::d(cell_size).shape(),
        'E' => upper::e(cell_size).shape(),
        'F' => upper::f(cell_size).shape(),
        'G' => upper::g(cell_size).shape(),
        'H' => upper::h(cell_size).shape(),
        'I' => upper::i(cell_size).shape(),
        'J' => upper::j(cell_size).shape(),
        'K' => upper::k(cell_size).shape(),
        'L' => upper::l(cell_size).shape(),
        'M' => upper::m(cell_size).shape(),
        'N' => upper::n(cell_size).shape(),
        'O' => upper::o(cell_size).shape(),
        'P' => upper::p(cell_size).shape(),
        'Q' => upper::q(cell_size).shape(),
        'R' => upper::r(cell_size).shape(),
        'S' => upper::s(cell_size).shape(),
        'T' => upper::t(cell_size).shape(),
        'U' => upper::u(cell_size).shape(),
        'V' => upper::v(cell_size).shape(),
        'W' => upper::w(cell_size).shape(),
        'X' => upper::x(cell_size).shape(),
        'Y' => upper::y(cell_size).shape(),
        'Z' => upper::z(cell_size).shape(),
        '.' => punct::period(cell_size).shape(),
        ',' => punct::comma(cell_size).shape(),
        '!' => punct::exclamation(cell_size).shape(),
        ' ' => punct::space().shape(),
        _ => unimplemented!(),
    }
}

pub enum Align {
    Left,
    Center,
    Right,
}

pub fn text(
    text: &str,
    align: Align,
    cell_size @ [cell_width, cell_height]: [f64; 2],
    _padding @ [pad_x, pad_y]: [f64; 2],
    modifier: Option<impl Fn(DynShape, [f64; 2], [f64; 2]) -> DynShape>,
) -> impl IntoShape {
    let total_size @ [total_width, total_height] = [cell_width + pad_x, cell_height + pad_y];

    let id_x = REPEAT_ID_2D.path().push(X).read();
    let id_x_lt = |t: f64| id_x.clone().lt(t);

    let id_y = REPEAT_ID_2D.path().push(Y).read();
    let id_y_lt = |t: f64| id_y.clone().lt(t);

    let lines = text.lines().map(|line| line.replace('\t', "    ")).rev();

    let max_width = lines
        .clone()
        .fold(0, |acc, line| acc.max(line.chars().count())) as f64;
    let total_max_x = max_width - 1.0;

    let height = lines.clone().count() as f64;
    let max_y = height - 1.0;

    lines
        .enumerate()
        .fold(Select::new(Infinity), |acc, (y, line)| {
            let line = line.replace('\t', "    ");
            let width = line.len() as f64;
            let max_x = width - 1.0;

            let field = line
                .chars()
                .enumerate()
                .fold(Select::new(Infinity), |acc, (x, char)| {
                    let char_field = char_field(char, cell_size);
                    let char_field = if let Some(modifier) = modifier.as_ref() {
                        modifier(char_field.shape(), cell_size, total_size)
                    } else {
                        char_field.shape()
                    };
                    acc.case(
                        id_x_lt(x as f64 + 1.0),
                        char_field.translate([x as f64 * total_width, 0.0]),
                    )
                })
                .prepass(
                    Infinity
                        .repeat_clamped(total_size, [0.0, 0.0], [max_x, 0.0])
                        .filter(REPEAT_ID_2D),
                )
                .shape();

            let field = match align {
                Align::Left => field,
                Align::Center | Align::Right => field
                    .translate([
                        match align {
                            Align::Center => (total_max_x - max_x) * 0.5,
                            Align::Right => total_max_x - max_x,
                            _ => unreachable!(),
                        },
                        0.0,
                    ])
                    .shape(),
            };

            acc.case(
                id_y_lt(y as f64 + 1.0),
                field.translate([0.0, y as f64 * total_height]),
            )
        })
        .prepass(
            Infinity
                .repeat_clamped(total_size, [0.0, 0.0], [0.0, max_y])
                .filter(REPEAT_ID_2D),
        )
        .translate([
            -total_max_x * total_width * 0.5,
            -max_y * total_height * 0.5,
        ])
}
