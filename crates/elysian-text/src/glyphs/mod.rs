use elysian_core::expr::IntoPath;
use elysian_ir::{
    ast::{COLOR, X, Y},
    module::{DynAsIR, IntoAsIR},
};
use elysian_shapes::combine::{Combinator, Combine};
use elysian_shapes::modify::IntoModify;
use elysian_shapes::{
    combine::Union,
    field::{Infinity, Point},
    filter::IntoFilter,
    modify::{IntoIsosurface, IntoManifold, IntoRepeat, IntoSet, IntoTranslate, REPEAT_ID_2D},
    scale::IntoScale,
    select::Select,
};

pub mod greek;
pub mod lower;
pub mod punct;
pub mod upper;

pub fn combinator() -> Combinator {
    Combinator::build().push(Union)
}

pub fn char_field(char: char, cell_size: [f64; 2]) -> impl IntoAsIR {
    match char {
        'a' => lower::a().as_ir(),
        'b' => lower::b().as_ir(),
        'c' => lower::c().as_ir(),
        'd' => lower::d().as_ir(),
        'e' => lower::e().as_ir(),
        'f' => lower::f().as_ir(),
        'g' => lower::g().as_ir(),
        'h' => lower::h().as_ir(),
        'i' => lower::i().as_ir(),
        'j' => lower::j().as_ir(),
        'k' => lower::k().as_ir(),
        'l' => lower::l().as_ir(),
        'm' => lower::m().as_ir(),
        'n' => lower::n().as_ir(),
        'o' => lower::o().as_ir(),
        'p' => lower::p().as_ir(),
        'q' => lower::q().as_ir(),
        'r' => lower::r().as_ir(),
        's' => lower::s().as_ir(),
        't' => lower::t().as_ir(),
        'u' => lower::u().as_ir(),
        'v' => lower::v().as_ir(),
        'w' => lower::w().as_ir(),
        'x' => lower::x().as_ir(),
        'y' => lower::y().as_ir(),
        'z' => lower::z().as_ir(),
        'A' => upper::a(cell_size).as_ir(),
        'B' => upper::b(cell_size).as_ir(),
        'C' => upper::c(cell_size).as_ir(),
        'D' => upper::d(cell_size).as_ir(),
        'E' => upper::e(cell_size).as_ir(),
        'F' => upper::f(cell_size).as_ir(),
        'G' => upper::g(cell_size).as_ir(),
        'H' => upper::h(cell_size).as_ir(),
        'I' => upper::i(cell_size).as_ir(),
        'J' => upper::j(cell_size).as_ir(),
        'K' => upper::k(cell_size).as_ir(),
        'L' => upper::l(cell_size).as_ir(),
        'M' => upper::m(cell_size).as_ir(),
        'N' => upper::n(cell_size).as_ir(),
        'O' => upper::o(cell_size).as_ir(),
        'P' => upper::p(cell_size).as_ir(),
        'Q' => upper::q(cell_size).as_ir(),
        'R' => upper::r(cell_size).as_ir(),
        'S' => upper::s(cell_size).as_ir(),
        'T' => upper::t(cell_size).as_ir(),
        'U' => upper::u(cell_size).as_ir(),
        'V' => upper::v(cell_size).as_ir(),
        'W' => upper::w(cell_size).as_ir(),
        'X' => upper::x(cell_size).as_ir(),
        'Y' => upper::y(cell_size).as_ir(),
        'Z' => upper::z(cell_size).as_ir(),
        '.' => punct::period(cell_size).as_ir(),
        ',' => punct::comma(cell_size).as_ir(),
        '!' => punct::exclamation(cell_size).as_ir(),
        ' ' => punct::space().as_ir(),
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
    cell_size: [f64; 2],
    padding: [f64; 2],
    modifier: Option<impl Fn(DynAsIR, [f64; 2], [f64; 2]) -> DynAsIR>,
) -> impl IntoAsIR {
    let total_size = [cell_size[0] + padding[0], cell_size[1] + padding[1]];

    let id_x = REPEAT_ID_2D.path().push(X).read();
    let id_x_lt = |t: f64| id_x.clone().lt(t);

    let id_y = REPEAT_ID_2D.path().push(Y).read();
    let id_y_lt = |t: f64| id_y.clone().lt(t);

    let lines = text.lines().map(|line| line.replace('\t', "    ")).rev();

    let total_width = lines
        .clone()
        .fold(0, |acc, line| acc.max(line.chars().count())) as f64;
    let total_max_x = total_width - 1.0;

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
                        modifier(char_field.as_ir(), cell_size, total_size)
                    } else {
                        char_field.as_ir()
                    };
                    acc.case(
                        id_x_lt(x as f64 + 1.0),
                        char_field.translate([x as f64 * total_size[0], 0.0]),
                    )
                })
                .modify()
                .push_pre(
                    Infinity
                        .repeat_clamped(total_size, [0.0, 0.0], [max_x, 0.0])
                        .filter(REPEAT_ID_2D),
                );

            let field = match align {
                Align::Left => field,
                Align::Center | Align::Right => field.translate([
                    match align {
                        Align::Center => (total_max_x - max_x) * 0.5,
                        Align::Right => total_max_x - max_x,
                        _ => unreachable!(),
                    },
                    0.0,
                ]),
            };

            acc.case(
                id_y_lt(y as f64 + 1.0),
                field.translate([0.0, y as f64 * total_size[1]]),
            )
        })
        .modify()
        .push_pre(
            Infinity
                .repeat_clamped(total_size, [0.0, 0.0], [0.0, max_y])
                .filter(REPEAT_ID_2D),
        )
        .translate([
            -total_max_x * total_size[0] * 0.5,
            -max_y * total_size[1] * 0.5,
        ])
}
