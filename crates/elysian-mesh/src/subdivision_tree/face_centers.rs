use crate::{
    bounds::Bounds,
    gltf_export::Points,
    subdivision_tree::{SubdivisionTree, SubdivisionCell},
    tree::Tree,
    vector_space::{VectorSpace, D2, D3},
};

pub trait FaceCenters<D: VectorSpace<f64>> {
    fn face_centers(&self) -> Points<D>;
}

impl FaceCenters<D2> for SubdivisionTree<D2> {
    fn face_centers(&self) -> Points<D2> {
        match self {
            Tree::Root([a, b, c, d]) => match [&**a, &**b, &**c, &**d] {
                [Tree::Leaf(_), Tree::Leaf(SubdivisionCell {
                    bounds:
                        Bounds {
                            min: bottom,
                            max: right,
                        },
                    ..
                }), Tree::Leaf(SubdivisionCell {
                    bounds:
                        Bounds {
                            min: left,
                            max: top,
                        },
                    ..
                }), Tree::Leaf(_)] => Points::from_iter([*bottom, *right, *left, *top]),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

impl FaceCenters<D3> for SubdivisionTree<D3> {
    fn face_centers(&self) -> Points<D3> {
        match self {
            Tree::Root([a, b, c, d, e, f, g, h]) => {
                match [&**a, &**b, &**c, &**d, &**e, &**f, &**g, &**h] {
                    [Tree::Leaf(_), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { max: right, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { max: top, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min: front, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { max: back, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min: bottom, .. },
                        ..
                    }), Tree::Leaf(SubdivisionCell {
                        bounds: Bounds { min: left, .. },
                        ..
                    }), Tree::Leaf(_)] => {
                        Points::from_iter([*right, *top, *front, *back, *bottom, *left])
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
