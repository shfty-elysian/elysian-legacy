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
