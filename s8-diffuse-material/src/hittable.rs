use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct Hittable<T: Shape> {
    pub shape: T,
}

pub trait Shape {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl Hittable<Sphere> {
    pub fn new(shape: Sphere) -> Self {
        Self { shape }
    }
}
