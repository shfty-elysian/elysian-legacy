use crate::{subdivision_tree::SubdivisionTree, vector_space::{D2, D3}};

pub trait CellIndices {
    const CELL_INDICES: &'static [u32];
}

impl CellIndices for SubdivisionTree<D2> {
    const CELL_INDICES: &'static [u32] = &[0, 2, 2, 3, 3, 1, 1, 0];
}

impl CellIndices for SubdivisionTree<D3> {
    const CELL_INDICES: &'static [u32] = &[
        0, 1, 1, 3, 2, 3, 2, 0, 4, 5, 5, 7, 6, 7, 6, 4, 0, 4, 1, 5, 2, 6, 3, 7,
    ];
}

