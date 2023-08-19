pub trait Vec2<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

impl<T> Vec2<T> for [T; 2]
where
    T: Copy,
{
    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }
}

pub trait Vec3<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

impl<T> Vec3<T> for [T; 3]
where
    T: Copy,
{
    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }

    fn z(&self) -> T {
        self[2]
    }
}

pub trait Unzip3<T, U, V>: Sized + IntoIterator<Item = (T, U, V)> {
    fn unzip3<W, X, Y>(self) -> (W, X, Y)
    where
        W: Default + Extend<T>,
        X: Default + Extend<U>,
        Y: Default + Extend<V>,
    {
        let (buffers, bufs): (W, Vec<_>) = self
            .into_iter()
            .map(|(buffer, view, accessor)| (buffer, (view, accessor)))
            .unzip();

        let (buffer_views, accessors): (X, Y) = bufs.into_iter().unzip();

        (buffers, buffer_views, accessors)
    }
}

impl<T, U, V, W> Unzip3<U, V, W> for T where T: IntoIterator<Item = (U, V, W)> {}
