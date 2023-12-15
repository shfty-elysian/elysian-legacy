use std::hash::{Hash, Hasher};

use crate::{
    bounds::Bounds,
    vector_space::{VectorSpace, D2, D3},
};

pub struct SubdivisionCell<D: VectorSpace<f64>> {
    pub bounds: Bounds<D>,
    pub ty: CellType,
}

impl<D> SubdivisionCell<D>
where
    D: VectorSpace<f64>,
{
    pub fn is_type(&self, ty: CellType) -> bool {
        self.ty == ty
    }

    pub fn is_empty(&self) -> bool {
        self.is_type(CellType::Empty)
    }

    pub fn is_contour(&self) -> bool {
        self.is_type(CellType::Contour)
    }

    pub fn is_full(&self) -> bool {
        self.is_type(CellType::Full)
    }
}

impl std::fmt::Debug for SubdivisionCell<D2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubdivisionCell")
            .field("bounds", &self.bounds)
            .field("ty", &self.ty)
            .finish()
    }
}

impl std::fmt::Debug for SubdivisionCell<D3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubdivisionCell")
            .field("bounds", &self.bounds)
            .field("ty", &self.ty)
            .finish()
    }
}

impl<D> Clone for SubdivisionCell<D>
where
    D: VectorSpace<f64>,
    D::DimensionVector: Clone,
{
    fn clone(&self) -> Self {
        Self {
            bounds: self.bounds.clone(),
            ty: self.ty.clone(),
        }
    }
}

impl<D> Copy for SubdivisionCell<D>
where
    D: VectorSpace<f64>,
    D::DimensionVector: Copy,
{
}

impl<D> PartialEq for SubdivisionCell<D>
where
    D: VectorSpace<f64>,
    D::DimensionVector: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.bounds == other.bounds && self.ty == other.ty
    }
}

impl<D> PartialOrd for SubdivisionCell<D>
where
    D: VectorSpace<f64>,
    D::DimensionVector: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ty.partial_cmp(&other.ty) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.bounds.partial_cmp(&other.bounds)
    }
}

impl<D> Hash for SubdivisionCell<D>
where
    D: VectorSpace<f64>,
    D::DimensionVector: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bounds.hash(state);
        self.ty.hash(state);
    }
}

/// State of a cell in an implicit surface sampling grid
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellType {
    /// Cell is fully outside the isosurface
    Empty,
    /// Cell intersects the isosurface
    Contour,
    /// Cell is fully inside the isosurface
    Full,
}
