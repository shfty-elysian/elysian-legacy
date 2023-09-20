pub trait CollectArray<T, const N: usize> {
    fn try_collect_array(self) -> Result<[T; N], <[T; N] as TryFrom<Vec<T>>>::Error>;

    fn collect_array(self) -> [T; N]
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        self.try_collect_array().unwrap()
    }
}

impl<T, U, const N: usize> CollectArray<U, N> for T
where
    T: Iterator<Item = U>,
{
    fn try_collect_array(self) -> Result<[U; N], <[U; N] as TryFrom<Vec<U>>>::Error> {
        self.collect::<Vec<_>>().try_into()
    }
}

#[macro_export]
macro_rules! derive_phantom_newtype {
    ($ident:ident, $gen:ident) => {
        impl<$gen> Clone for $ident<$gen> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), self.1.clone())
            }
        }

        impl<$gen> Copy for $ident<$gen> {}

        impl<$gen> PartialOrd for $ident<$gen> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self.0.partial_cmp(&other.0) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                self.1.partial_cmp(&other.1)
            }
        }

        impl<$gen> Ord for $ident<$gen> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0).then(self.1.cmp(&other.1))
            }
        }

        impl<$gen> std::hash::Hash for $ident<$gen> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
                self.1.hash(state);
            }
        }
    };
}

