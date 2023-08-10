use elysian_core::{
    ast::expr::{Expr, IntoExpr, IntoLiteral, IntoRead},
    ir::ast::{DISTANCE, ERROR, GRADIENT_2D, NORMAL, POSITION_2D, POSITION_3D, UV, X, Y, Z},
};

use crate::{derive_support_vector::SUPPORT_VECTOR_2D, modify::REPEAT_ID_2D, voronoi::CELL_ID};

pub fn distance_color(fac: f64) -> Expr {
    let color = 1.0.literal() - (DISTANCE.prop().read() * fac).clamp(0.0, 1.0);
    Expr::vector4(color.clone(), color.clone(), color, 1.0.literal())
}

pub fn normal_color() -> Expr {
    Expr::vector4(
        NORMAL.path().push(X).read() * 0.5 + 0.5,
        NORMAL.path().push(Y).read() * 0.5 + 0.5,
        NORMAL.path().push(Z).read() * 0.5 + 0.5,
        1.0,
    )
}

pub fn directional_light_color(dir: Expr) -> Expr {
    Expr::vector4(
        -NORMAL.path().read().dot(dir.clone()),
        -NORMAL.path().read().dot(dir.clone()),
        -NORMAL.path().read().dot(dir),
        1.0,
    )
}

pub fn ambient_light_color(ambient: impl IntoExpr) -> Expr {
    let ambient = ambient.expr();
    Expr::vector4(ambient.clone(), ambient.clone(), ambient.clone(), 1.0)
}

pub fn gradient_color() -> Expr {
    Expr::vector4(
        GRADIENT_2D.path().push(X).read() * 0.5 + 0.5,
        GRADIENT_2D.path().push(Y).read() * 0.5 + 0.5,
        0.0,
        1.0,
    )
}

pub fn uv_color() -> Expr {
    Expr::vector4(UV.path().push(X).read(), UV.path().push(Y).read(), 0.0, 1.0)
}

pub fn position_2d_color() -> Expr {
    Expr::vector4(
        POSITION_2D.path().push(X).read(),
        POSITION_2D.path().push(Y).read(),
        0.0,
        1.0.literal(),
    )
}

pub fn position_3d_color() -> Expr {
    Expr::vector4(
        POSITION_3D.path().push(X).read(),
        POSITION_3D.path().push(Y).read(),
        POSITION_3D.path().push(Z).read(),
        1.0.literal(),
    )
}

pub fn cell_id_color(count: usize) -> Expr {
    let fac = 1.0 / count as f64;
    Expr::vector4(
        CELL_ID.prop().read() * fac,
        CELL_ID.prop().read() * fac,
        CELL_ID.prop().read() * fac,
        1.0,
    )
}

pub fn repeat_id_color(count: usize) -> Expr {
    let fac = 1.0 / count as f64;
    Expr::vector4(
        REPEAT_ID_2D.path().push(X).read().abs() * fac,
        REPEAT_ID_2D.path().push(Y).read().abs() * fac,
        0.0,
        1.0,
    )
}

pub fn support_vector_color() -> Expr {
    Expr::vector4(
        SUPPORT_VECTOR_2D.path().push(X).read() * 0.5 + 0.5,
        SUPPORT_VECTOR_2D.path().push(Y).read() * 0.5 + 0.5,
        0.0,
        1.0,
    )
}

pub fn error_color() -> Expr {
    Expr::vector4(
        ERROR.prop().read().abs(),
        ERROR.prop().read().min(0.0).abs(),
        0.0,
        1.0,
    )
}
