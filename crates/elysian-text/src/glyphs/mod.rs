use elysian_core::{
    ast::{combine::Combinator, filter::IntoFilter, modify::IntoModify, select::Select},
    ir::{
        ast::{DISTANCE, GRADIENT_2D, UV, X, Y},
        module::IntoAsIR,
    },
};
use elysian_shapes::{
    combine::{SmoothUnion, Union},
    field::Infinity,
    modify::{IntoRepeat, IntoTranslate, REPEAT_ID_2D},
    scale::IntoScale,
};

pub mod greek;
pub mod lower;
pub mod punct;
pub mod upper;

pub fn combinator() -> Combinator {
    Combinator::build()
        .push(Union)
}

pub fn char_field(char: char) -> impl IntoAsIR {
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
        'A' => upper::a().as_ir(),
        'B' => upper::b().as_ir(),
        'C' => upper::c().as_ir(),
        'D' => upper::d().as_ir(),
        'E' => upper::e().as_ir(),
        'F' => upper::f().as_ir(),
        'G' => upper::g().as_ir(),
        'H' => upper::h().as_ir(),
        'I' => upper::i().as_ir(),
        'J' => upper::j().as_ir(),
        'K' => upper::k().as_ir(),
        'L' => upper::l().as_ir(),
        'M' => upper::m().as_ir(),
        'N' => upper::n().as_ir(),
        'O' => upper::o().as_ir(),
        'P' => upper::p().as_ir(),
        'Q' => upper::q().as_ir(),
        'R' => upper::r().as_ir(),
        'S' => upper::s().as_ir(),
        'T' => upper::t().as_ir(),
        'U' => upper::u().as_ir(),
        'V' => upper::v().as_ir(),
        'W' => upper::w().as_ir(),
        'X' => upper::x().as_ir(),
        'Y' => upper::y().as_ir(),
        'Z' => upper::z().as_ir(),
        '.' => punct::period().as_ir(),
        ',' => punct::comma().as_ir(),
        '!' => punct::exclamation().as_ir(),
        ' ' => punct::space().as_ir(),
        _ => unimplemented!(),
    }
}

pub enum Align {
    Left,
    Center,
    Right,
}

pub fn text(text: &str, align: Align, cell_size: [f64; 2], scale: f64) -> impl IntoAsIR {
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
                    let field = char_field(char).scale(scale);
                    acc.case(
                        id_x_lt(x as f64 + 1.0),
                        field.translate([x as f64 * cell_size[0], 0.0]),
                    )
                })
                .modify()
                .push_pre(
                    Infinity
                        .repeat_clamped(cell_size, [0.0, 0.0], [max_x, 0.0])
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
                field.translate([0.0, y as f64 * cell_size[1]]),
            )
        })
        .modify()
        .push_pre(
            Infinity
                .repeat_clamped(cell_size, [0.0, 0.0], [0.0, max_y])
                .filter(REPEAT_ID_2D),
        )
        .translate([-total_max_x * cell_size[0] * 0.5, -max_y * cell_size[1] * 0.5])
}
