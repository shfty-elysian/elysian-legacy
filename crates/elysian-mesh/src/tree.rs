use std::fmt::Debug;

/// Recursive tree with N cells per root
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tree<T, const N: usize> {
    Root([Box<Tree<T, N>>; N]),
    Leaf(T),
}

pub type Tree2<T> = Tree<T, 2>;
pub type Tree4<T> = Tree<T, 4>;
pub type Tree8<T> = Tree<T, 8>;

impl<T, const N: usize> Tree<T, N> {
    // Return the maximum depth present in the tree
    pub fn depth(&self) -> usize {
        self.map_depth_impl(0)
            .into_iter()
            .fold(0, |acc, next| acc.max(next))
    }

    // Map the tree to one containing a depth at each leaf
    pub fn map_depth(&self) -> Tree<usize, N> {
        self.map_depth_impl(0)
    }

    fn map_depth_impl(&self, d: usize) -> Tree<usize, N> {
        match self {
            Tree::Leaf(_) => Tree::Leaf(d),
            Tree::Root(t) => Tree::Root(
                t.into_iter()
                    .map(|t| Box::new(t.map_depth_impl(d + 1)))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ),
        }
    }

    // Return the maximum number of cells needed to respresent a tree of this depth
    pub fn resolution(&self) -> usize {
        N.pow(self.depth() as u32)
    }

    // Conversion to borrowing iterator
    pub fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        match self {
            Self::Leaf(t) => Box::new(std::iter::once(t)),
            Self::Root(t) => Box::new(t.into_iter().flat_map(|t| t.iter())),
        }
    }

    // Structure-preserving map
    pub fn map<F, U>(self, f: F) -> Tree<U, N>
    where
        F: Clone + Fn(T) -> U,
        U: Debug,
    {
        match self {
            Tree::Leaf(t) => Tree::Leaf(f(t)),
            Tree::Root(t) => Tree::Root({
                let v: Vec<_> = t.into_iter().map(|t| Box::new(t.map(f.clone()))).collect();
                v.try_into().unwrap()
            }),
        }
    }
}

impl<T, const N: usize> IntoIterator for Tree<T, N>
where
    T: 'static,
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

