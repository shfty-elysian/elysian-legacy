use std::fmt::Debug;

use crate::vector_space::{SubdivisionArray, D1, D2, D3};

/// Recursive tree with N cells per root
#[derive(Clone, PartialEq, PartialOrd)]
pub enum Tree<T, D: SubdivisionArray<Box<Tree<T, D>>>> {
    Root(D::SubdivisionArray),
    Leaf(T),
}

impl<T> Debug for Tree<T, D2>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(arg0) => f.debug_tuple("Root").field(arg0).finish(),
            Self::Leaf(arg0) => f.debug_tuple("Leaf").field(arg0).finish(),
        }
    }
}

impl<T> Debug for Tree<T, D3>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(arg0) => f.debug_tuple("Root").field(arg0).finish(),
            Self::Leaf(arg0) => f.debug_tuple("Leaf").field(arg0).finish(),
        }
    }
}

pub type Tree2<T> = Tree<T, D1>;
pub type Tree4<T> = Tree<T, D2>;
pub type Tree8<T> = Tree<T, D3>;

impl<T, D> Tree<T, D>
where
    D: SubdivisionArray<Box<Tree<T, D>>> + SubdivisionArray<Box<Tree<usize, D>>>,
    for<'a> &'a <D as SubdivisionArray<Box<Tree<T, D>>>>::SubdivisionArray:
        IntoIterator<Item = &'a Box<Tree<T, D>>>,
    <<D as SubdivisionArray<Box<Tree<usize, D>>>>::SubdivisionArray as IntoIterator>::IntoIter:
        'static,
{
    // Return the maximum depth present in the tree
    pub fn depth(&self) -> usize {
        self.map_depth_impl(0)
            .into_iter()
            .fold(0, |acc, next| acc.max(next))
    }

    // Map the tree to one containing a depth at each leaf
    pub fn map_depth(&self) -> Tree<usize, D> {
        self.map_depth_impl(0)
    }

    fn map_depth_impl(&self, d: usize) -> Tree<usize, D> {
        match self {
            Tree::Leaf(_) => Tree::Leaf(d),
            Tree::Root(t) => Tree::Root(
                t.into_iter()
                    .map(|t| Box::new(t.map_depth_impl(d + 1)))
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()
                    .expect("Invalid root length"),
            ),
        }
    }

    // Return the maximum number of cells needed to respresent a tree of this depth
    pub fn resolution(&self) -> usize {
        D::SUBDIVISION.pow(self.depth() as u32)
    }

    // Conversion to borrowing iterator
    pub fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        match self {
            Self::Leaf(t) => Box::new(std::iter::once(t)),
            Self::Root(t) => Box::new(t.into_iter().flat_map(|t| t.iter())),
        }
    }

    // Structure-preserving map
    pub fn map<F, U>(self, f: F) -> Tree<U, D>
    where
        F: Clone + Fn(T) -> U,
        U: Debug,
        D: SubdivisionArray<Box<Tree<T, D>>> + SubdivisionArray<Box<Tree<U, D>>>,
        <D as SubdivisionArray<Box<Tree<U, D>>>>::SubdivisionArray: Debug,
    {
        match self {
            Tree::Leaf(t) => Tree::Leaf(f(t)),
            Tree::Root(t) => Tree::Root({
                let v: Vec<_> = t
                    .into_iter()
                    .map(|u: Box<Tree<T, D>>| Box::new(u.map(f.clone())))
                    .collect();
                v.try_into().ok().expect("Invalid root length")
            }),
        }
    }
}

impl<T, D> IntoIterator for Tree<T, D>
where
    T: 'static,
    D: SubdivisionArray<Box<Tree<T, D>>>,
    <<D as SubdivisionArray<Box<Tree<T, D>>>>::SubdivisionArray as IntoIterator>::IntoIter: 'static,
{
    type Item = T;

    type IntoIter = Box<dyn Iterator<Item = T>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Tree::Leaf(t) => Box::new(std::iter::once(t)),
            Tree::Root(t) => Box::new(t.into_iter().flat_map(|t| t.into_iter())),
        }
    }
}

