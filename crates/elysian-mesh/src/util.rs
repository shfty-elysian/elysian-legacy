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

pub fn center((min, max): ([f64; 2], [f64; 2])) -> [f64; 2] {
    [(min.x() + max.x()) / 2.0, (min.y() + max.y()) / 2.0]
}

