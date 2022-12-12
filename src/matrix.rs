use core::{
    fmt::{Display, Formatter, Write},
    ops::{Add, Div, DivAssign, Mul, Neg, Sub},
};

use alloc::fmt;

use crate::{fixed::FixedI32, vec3::Vec3};

pub type Matrix3x3FI32 = Matrix3x3<FixedI32>;

#[derive(Copy, Clone, Default)]
pub struct Matrix3x3<T>
where
    T: Copy + Clone + Default,
{
    pub state: [T; 9],
}

impl<T> Matrix3x3<T>
where
    T: Copy + Clone + Default,
{
    pub fn new(state: [T; 9]) -> Self {
        Self { state }
    }

    pub fn det(self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
    {
        self.state[0] * self.state[4] * self.state[8]
            + self.state[1] * self.state[5] * self.state[6]
            + self.state[2] * self.state[3] * self.state[7]
            - self.state[2] * self.state[4] * self.state[6]
            - self.state[1] * self.state[3] * self.state[8]
            - self.state[0] * self.state[5] * self.state[7]
    }

    #[allow(clippy::many_single_char_names)]
    pub fn invert(self) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + DivAssign,
    {
        let a = self.state[4] * self.state[8] - self.state[5] * self.state[7];
        let b = -(self.state[3] * self.state[8] - self.state[5] * self.state[6]);
        let c = self.state[3] * self.state[7] - self.state[4] * self.state[6];

        let d = -(self.state[1] * self.state[8] - self.state[2] * self.state[7]);
        let e = self.state[0] * self.state[8] - self.state[2] * self.state[6];
        let f = -(self.state[0] * self.state[7] - self.state[1] * self.state[6]);

        let g = self.state[1] * self.state[5] - self.state[2] * self.state[4];
        let h = -(self.state[0] * self.state[5] - self.state[2] * self.state[3]);
        let i = self.state[0] * self.state[4] - self.state[1] * self.state[3];

        Matrix3x3::new([a, d, g, b, e, h, c, f, i]) / self.det()
    }
}

impl<T> Display for Matrix3x3<T>
where
    T: Display + Copy + Clone + Default,
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "{{ [ {}, {}, {} ] , [ {}, {}, {} ], [ {}, {}, {} ] }}",
            self.state[0],
            self.state[3],
            self.state[6],
            self.state[1],
            self.state[4],
            self.state[7],
            self.state[2],
            self.state[5],
            self.state[8],
        ))
    }
}

impl<T> Div<T> for Matrix3x3<T>
where
    T: DivAssign + Copy + Clone + Default,
{
    type Output = Self;

    fn div(mut self, rhs: T) -> Self::Output {
        for elem in self.state.iter_mut() {
            *elem /= rhs;
        }

        self
    }
}

// Matrix-vector multiplication
impl<T> Mul<Vec3<T>> for Matrix3x3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Clone + Default,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(
            rhs.x * self.state[0] + rhs.y * self.state[1] + rhs.z * self.state[2],
            rhs.x * self.state[3] + rhs.y * self.state[4] + rhs.z * self.state[5],
            rhs.x * self.state[6] + rhs.y * self.state[7] + rhs.z * self.state[8],
        )
    }
}
