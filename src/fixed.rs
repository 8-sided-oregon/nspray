// If you wake up in a house that's full of smoke: don't panic, call me and I'll tell you a joke.

//extern crate ndless_handler;

use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use alloc::{
    format,
    string::{String, ToString},
};
use ndless::prelude::Float;
use oorandom::Rand32;

pub const PI: FixedI32 = FixedI32::from_dec(3, 14159, 5);

#[macro_export]
macro_rules! fxi32 {
    ($element:expr) => {
        $crate::fixed::FixedI32::from($element)
    };
}

/// A fixed point encoded i32 that has 16 bits of percision:
/// 0 1 0 1 0 1 0 0 0 0 0 1 0 0 0 0 . 1 0 1 0 1 0 0 0 0 0 1 0 0 0 0 0
/// = 86082 + 321/512
/// This is neccessary (i think), because the TI Nspire doesn't have a native FPU. Or atleast, I
/// think, not that I can check because the people at TI are all a bunch of fucking prudes.
#[derive(Clone, Copy, Debug)]
pub struct FixedI32 {
    value: i32,
}

impl FixedI32 {
    pub fn new(value: i32) -> Self {
        Self { value: value << 16 }
    }

    pub const fn from_components(whole: i16, frac: u16) -> Self {
        Self {
            value: ((whole as i32) << 16) | frac as i32,
        }
    }

    pub const fn from_dec(whole: i16, frac: u16, frac_mag: u8) -> Self {
        Self {
            value: ((whole as i32) << 16) | (((frac as i32) << 16) / 10i32.pow(frac_mag as u32)),
        }
    }

    pub fn modulo(self, modulus: Self) -> Self {
        Self {
            value: self.value % modulus.value,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
        }
    }

    /// Reasonably accurate tangent approximation.
    pub fn tan(self) -> Self {
        self.sin() / self.cos()
    }

    /// Reasonably accurate sine approximation.
    pub fn sin(self) -> Self {
        let mut sign = self.value.signum();
        let mut r = (self * sign).modulo(PI * 2);

        if r >= PI {
            sign *= -1;
            r -= PI;
        }

        if r >= PI / 2 {
            r = PI - r;
        }

        r - r.pow(3) / 6 + r.pow(5) / 120 - r.pow(7) / 5040 * sign
    }

    /// Reasonably accurate cosine approximation.
    pub fn cos(self) -> Self {
        (PI / 2 + self).sin()
    }

    pub fn pow(self, n: u32) -> Self {
        let mut accum = FixedI32::from(1);

        for _ in 0..n {
            accum *= self;
        }

        accum
    }

    pub fn sqrt(self) -> Self {
        // Just a simple bit of newton's method, quite slow.
        let a_half = self / 2;
        let mut x_n = FixedI32::from(1);

        for _ in 0..5 {
            x_n = x_n / 2 + a_half / x_n;
        }

        x_n
    }

    pub fn clamp(self, begin: i32, end: i32) -> Self {
        let w = self.value >> 16;

        if w <= begin {
            Self { value: begin << 16 }
        } else if w >= end {
            Self { value: end << 16 }
        } else {
            self
        }
    }

    /// Generates a new FixedI32 in the range of [0, 1)
    pub fn rand(rng: &mut Rand32) -> Self {
        Self {
            value: (rng.rand_u32() & 0xffff) as i32,
        }
    }
}

impl Into<i32> for FixedI32 {
    fn into(self) -> i32 {
        self.value >> 16
    }
}

impl From<&str> for FixedI32 {
    fn from(mut val: &str) -> Self {
        let (mut frac, mut frac_mag) = (0, 0);

        if let Some(index) = val.find('.') {
            if index != val.len() - 1 {
                frac = val[(index + 1)..].parse().unwrap();
                frac_mag = val.len() - index - 1;
            }

            val = &val[0..index];
        }

        let frac = (2 << 15) * frac / 10u32.pow(frac_mag as u32);
        let value = (val.parse::<i32>().unwrap() << 16) | frac as i32;

        Self { value }
    }
}

impl From<f32> for FixedI32 {
    fn from(val: f32) -> Self {
        Self {
            value: (val as i32) << 16 | ((val.fract().abs() * 2f32.powi(16)) as i32),
        }
    }
}

impl From<i64> for FixedI32 {
    fn from(value: i64) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl From<u64> for FixedI32 {
    fn from(value: u64) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl From<i32> for FixedI32 {
    fn from(value: i32) -> Self {
        Self { value: value << 16 }
    }
}

impl From<u32> for FixedI32 {
    fn from(value: u32) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl From<i16> for FixedI32 {
    fn from(value: i16) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl From<u16> for FixedI32 {
    fn from(value: u16) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl From<i8> for FixedI32 {
    fn from(value: i8) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl From<u8> for FixedI32 {
    fn from(value: u8) -> Self {
        Self {
            value: (value as i32) << 16,
        }
    }
}

impl Display for FixedI32 {
    // cheating lol
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let frac = self.value & 0xffff;
        let mut frac_str = String::new();

        if frac != 0 {
            frac_str = format!("{:.5}", (frac as f64) / 2f64.powi(16))[1..].to_string();
        }

        f.write_fmt(format_args!("{}{}", self.value >> 16, frac_str))
    }
}

impl Default for FixedI32 {
    fn default() -> Self {
        Self {
            value: i32::default(),
        }
    }
}

impl PartialEq for FixedI32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for FixedI32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Eq for FixedI32 {}

impl Ord for FixedI32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

// TODO: Implement these operations all in ARM Assembly.

impl Add for FixedI32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Add<i32> for FixedI32 {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value + (rhs << 16),
        }
    }
}

impl AddAssign for FixedI32 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for FixedI32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl Sub<i32> for FixedI32 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value - (rhs << 16),
        }
    }
}

impl SubAssign for FixedI32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Neg for FixedI32 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.value *= -1;

        self
    }
}

impl Mul for FixedI32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = self.value as i64 * rhs.value as i64;
        let adj = (res >> 16) as i32;

        Self { value: adj }
    }
}

impl Mul<i32> for FixedI32 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value * rhs,
        }
    }
}

impl MulAssign for FixedI32 {
    fn mul_assign(&mut self, rhs: Self) {
        let res = self.value as i64 * rhs.value as i64;
        self.value = (res >> 16) as i32;
    }
}

impl Div for FixedI32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let res = ((self.value as i64) << 16) / rhs.value as i64;

        Self { value: res as i32 }
    }
}

impl Div<i32> for FixedI32 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value / rhs,
        }
    }
}

impl DivAssign for FixedI32 {
    fn div_assign(&mut self, rhs: Self) {
        let res = ((self.value as i64) << 16) / rhs.value as i64;
        self.value = res as i32;
    }
}
