use alloc::{boxed::Box, format, rc::Rc, vec::Vec};

use crate::{
    dprintln,
    fixed::FixedI32,
    fxi32,
    material::Material,
    matrix::{Matrix3x3, Matrix3x3FI32},
    ray::Ray,
    vec3::Vec3FI32,
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: Vec3FI32,
    pub mapped_point: Option<Vec3FI32>,
    pub normal: Vec3FI32,
    pub t: FixedI32,
    pub front: bool,
    pub attenuation: Vec3FI32,
    pub material: Option<Rc<dyn Material>>,
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
    material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3FI32, radius: FixedI32, material: Option<Rc<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, record: &mut HitRecord, t_min: FixedI32, t_max: FixedI32) -> bool {
        let oc = ray.origin() - self.center;

        let a = ray.dir().mag_squared();
        let half_b = oc.dot(ray.dir());
        let c = oc.mag_squared() - self.radius * self.radius;

        // lol
        #[allow(clippy::suspicious_operation_groupings)]
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

        record.material = self.material.clone();
        record.mapped_point = None;

        true
    }
}

pub struct Plane {
    center: Vec3FI32,
    normal: Vec3FI32,
    inverse: Matrix3x3FI32,
    material: Option<Rc<dyn Material>>,
}

impl Plane {
    pub fn new(
        center: Vec3FI32,
        v: Vec3FI32,
        u: Vec3FI32,
        material: Option<Rc<dyn Material>>,
    ) -> Self {
        let normal = v.cross(u);
        let inverse =
            Matrix3x3FI32::new([v.x, u.x, normal.x, v.y, u.y, normal.y, v.z, u.z, normal.z]);

        dprintln!(
            "v: {v}, u: {u}, n: {normal}, mat: {inverse} inv: {}",
            inverse.invert()
        );
        let inverse = inverse.invert();

        Self {
            center,
            normal,
            inverse,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, record: &mut HitRecord, t_min: FixedI32, t_max: FixedI32) -> bool {
        let denom = ray.dir().dot(self.normal);

        if denom.abs() < fxi32!(0.01) {
            return false;
        }

        let t = (self.center - ray.origin()).dot(self.normal) / denom;

        if t < t_min || t > t_max {
            return false;
        }

        record.t = t;
        record.point = ray.at(t);
        record.mapped_point = Some(self.inverse * (record.point - self.center));
        record.set_face_normal(ray, self.normal);
        record.material = self.material.clone();

        //dprintln!("Mapped: {}", record.mapped_point.unwrap());

        true
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, record: &mut HitRecord, t_min: FixedI32, t_max: FixedI32) -> bool {
        let mut min_dist = t_max;
        let mut has_hit = false;
        let mut tmp_rec = HitRecord::default();

        for object in self.objects.iter() {
            if object.hit(ray, &mut tmp_rec, t_min, t_max) {
                has_hit = true;
                if tmp_rec.t < min_dist {
                    *record = tmp_rec.clone();
                    min_dist = record.t;
                }
            }
        }

        has_hit
    }
}
