use crate::marching_cells::{Corner, Corners, Face, ToCorners};

pub trait HasSignChange<D> {
    fn has_sign_change(&self, side: &Face<D>) -> bool;
}

impl<D> HasSignChange<D> for Corners<D>
where
    Face<D>: ToCorners<D>,
    Corners<D>: IntoIterator<Item = Corner<D>>,
    D: Copy,
{
    fn has_sign_change(&self, side: &Face<D>) -> bool {
        side.to_corners()
            .into_iter()
            .fold(false, |acc, next| acc | (*self & next).is_empty())
    }
}

