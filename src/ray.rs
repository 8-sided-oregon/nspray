use crate::{fixed::FixedI32, vec3::Vec3FI32};

pub struct Ray {
    origin: Vec3FI32,
    dir: Vec3FI32,
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
