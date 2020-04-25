use std::ops::*;
use std::fmt::*;
use num_traits::cast::{NumCast, cast};
use num_traits::sign::Signed;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

impl<T> Add for Vec2<T> where
    T: Add<T> {
    type Output = Vec2<<T as Add>::Output>;
    fn add(self, rhs: Vec2<T>) -> Vec2<<T as Add>::Output> {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> Sub for Vec2<T> where
    T: Sub<T> {
    type Output = Vec2<<T as Sub>::Output>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<<T as Sub>::Output> {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T> Mul for Vec2<T> where
    T: Mul<T> {
    type Output = Vec2<<T as Mul>::Output>;
    fn mul(self, rhs: Vec2<T>) -> Vec2<<T as Mul>::Output> {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T> Div for Vec2<T> where
    T: Div<T> {
    type Output = Vec2<<T as Div>::Output>;
    fn div(self, rhs: Vec2<T>) -> Vec2<<T as Div>::Output> {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl<T: AddAssign<T>> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: MulAssign<T>> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Add<T, Output=T> + Copy> Add<T> for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl<T: Sub<T, Output=T> + Copy> Sub<T> for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl<T: Mul<T, Output=T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Div<T, Output=T> + Copy> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl<T: NumCast> Vec2<T> {
    pub fn convert<U: NumCast>(self) -> Vec2<U> {
        Vec2 {
            x: cast(self.x).unwrap(),
            y: cast(self.y).unwrap()
        }
    }
}

#[allow(dead_code)]
impl<T: Neg<Output=T>> Vec2<T> {
    pub fn inv_x(self) -> Vec2<T> {
        Vec2 {
            x: -self.x,
            y: self.y
        }
    }

    pub fn inv_y(self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: -self.y
        }
    }
}

impl<T: Signed> Vec2<T> {
    pub fn abs(self) -> Self {
        Vec2 {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
