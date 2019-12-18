use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use num::{One, Signed, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

macro_rules! point2D {
    ($x:expr, $y:expr) => {
        Point2D::with_coordinates($x, $y)
    };
}

impl<T> Point2D<T> {
    pub fn with_coordinates(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Zero> Point2D<T> {
    pub fn origin() -> Self {
        Self::with_coordinates(T::zero(), T::zero())
    }
}

impl<T: One + Zero> Point2D<T> {
    pub fn x_basis() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn y_basis() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }
}

impl<T: Ord> Ord for Point2D<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            x => x,
        }
    }
}

impl<T: Ord> PartialOrd for Point2D<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for Point2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Point2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Point2D<T>
where
    T: Ord + Add<Output = T> + Sub<Output = T> + Signed,
{
    pub fn manhattan_distance(self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
