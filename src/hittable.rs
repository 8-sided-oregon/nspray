use crate::{fixed::FixedI32, fxi32, ray::Ray, vec3::Vec3FI32};

#[derive(Default)]
pub struct HitRecord {
    pub point: Vec3FI32,
    pub normal: Vec3FI32,
    pub t: FixedI32,
    pub front: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3FI32) {
        self.front = ray.dir().dot(outward_normal) < fxi32!(0);
        self.normal = if self.front {
            outward_normal
        } else {
            outward_normal * fxi32!(-1)
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, record: &mut HitRecord, t_min: FixedI32, t_max: FixedI32) -> bool;
}

#[derive(Default)]
pub struct Sphere {
    center: Vec3FI32,
    radius: FixedI32,
}

impl Sphere {
    pub fn new(center: Vec3FI32, radius: FixedI32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, record: &mut HitRecord, t_min: FixedI32, t_max: FixedI32) -> bool {
        let oc = ray.origin() - self.center;

        let a = ray.dir().mag_squared();
        let half_b = oc.dot(ray.dir());
        let c = oc.mag_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < fxi32!(0) {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(root);

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        true
    }
}
