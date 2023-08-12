use crate::{combine::Combine, shape::IntoShape};
use elysian_core::{expr::IntoExpr, identifier::Identifier};
use elysian_ir::{
    module::{NumericType, Type},
    property,
};

use crate::{
    combine::Union,
    field::Point,
    modify::{IntoSet, IntoTranslate},
};

pub const CELL_ID: Identifier = Identifier::new("partition_id", 1485962089216017275);
property!(CELL_ID, CELL_ID_PROP, Type::Number(NumericType::Float));

/// Given a list of points, produce a voronoi field with indexed cells.
pub fn voronoi(points: impl IntoIterator<Item = impl IntoExpr>) -> impl IntoShape {
    voronoi_id(points.into_iter().enumerate())
}

/// Given a list of IDs and points, produce a voronoi field with indexed cells.
pub fn voronoi_id(points: impl IntoIterator<Item = (usize, impl IntoExpr)>) -> impl IntoShape {
    points
        .into_iter()
        .fold(Combine::from(Union), |acc, (i, next)| {
            acc.push(Point.translate(next.expr()).set_pre(CELL_ID, i as f32))
        })
}

/// Given a list of IDs and points, produce a voronoi field with indexed cells.
pub fn voronoi_shapes(points: impl IntoIterator<Item = (usize, impl IntoShape)>) -> impl IntoShape {
    points
        .into_iter()
        .fold(Combine::from(Union), |acc, (i, next)| {
            acc.push(next.set_pre(CELL_ID, i as f32))
        })
}
