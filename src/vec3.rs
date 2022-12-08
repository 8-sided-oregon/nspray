use core::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use alloc::format;
use oorandom::Rand32;

use crate::{fixed::FixedI32, fxi32};

pub type Vec3FI32 = Vec3<FixedI32>;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn mag_squared(self) -> T
    where
        T: Mul<Output = T> + Add<Output = T>,
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, rhs: Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;

        Self { x, y, z }
    }
}

impl Vec3<FixedI32> {
    pub fn random_in_unit_sphere(rand: &mut Rand32) -> Self {
        loop {
            let v = Self {
                x: FixedI32::rand(rand),
                y: FixedI32::rand(rand),
                z: FixedI32::rand(rand),
            };

            if v.mag_squared() <= fxi32!(1) {
                return v;
            }
        }
    }

    pub fn mag(self) -> FixedI32 {
        self.mag_squared().sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.mag()
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < fxi32!(0.001) && self.y.abs() < fxi32!(0.001) && self.z.abs() < fxi32!(0.001)
    }
}

impl<T> From<T> for Vec3<T>
where
    T: Copy + Clone,
{
    fn from(val: T) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
        }
    }
}

impl<T> Display for Vec3<T>
where
    T: Display,
{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("[ {}, {}, {} ]", self.x, self.y, self.z))
    }
}

// TODO: Macroooooooos

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<T> for Vec3<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub<T> for Vec3<T>
where
    T: Sub<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign for Vec3<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> Div for Vec3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign for Vec3<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
