use crate::marching_cells::{Face, Corners, ToCorners, Corner};

pub trait Neighbours<D> {
    fn neighbours(&self, rhs: &Self, side: &Face<D>) -> bool;
}

impl<D> Neighbours<D> for Corners<D>
where
    Face<D>: ToCorners<D>,
    Corners<D>: IntoIterator<Item = Corner<D>>,
    D: Copy,
{
    fn neighbours(&self, rhs: &Self, side: &Face<D>) -> bool {
        let corners = side.to_corners();
        corners
            .into_iter()
            .zip((!corners).into_iter())
            .fold(true, |acc, (from, to)| {
                acc & ((*self & from).is_empty() == (*rhs & to).is_empty())
            })
    }
}

