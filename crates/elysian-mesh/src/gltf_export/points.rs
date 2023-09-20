use crate::vector_space::VectorSpace;

#[derive(Debug, Clone)]
pub struct Points<D>(Vec<D::DimensionVector>)
where
    D: VectorSpace<f64>;

impl<D> FromIterator<D::DimensionVector> for Points<D>
where
    D: VectorSpace<f64>,
{
    fn from_iter<T: IntoIterator<Item = D::DimensionVector>>(iter: T) -> Self {
        Points(iter.into_iter().collect())
    }
}

impl<D> IntoIterator for Points<D>
where
    D: VectorSpace<f64>,
{
    type Item = D::DimensionVector;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

