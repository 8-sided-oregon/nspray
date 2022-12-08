use core::fmt::{self, Display, Formatter};

use crate::{fixed::FixedI32, vec3::Vec3FI32};

#[derive(Debug)]
pub struct Ray {
    origin: Vec3FI32,
    dir: Vec3FI32,
}

impl Display for Ray {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "[ origin: {}, dir: {} ]",
            self.origin, self.dir
        ))
    }
}

impl Ray {
    pub fn new(origin: Vec3FI32, dir: Vec3FI32) -> Ray {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Vec3FI32 {
        self.origin
    }

    pub fn dir(&self) -> Vec3FI32 {
        self.dir
    }

    pub fn at(&self, t: FixedI32) -> Vec3FI32 {
        self.origin + self.dir * t
    }
}
