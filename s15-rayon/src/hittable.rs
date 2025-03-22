use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
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

impl Hittable {
    pub fn new<T: 'static + Shape, U: 'static + Material>(shape: T, material: U) -> Self {
        Self {
            shape: Arc::new(shape),
            material: Arc::new(material),
        }
    }
}

pub struct Hittable {
    pub shape: Arc<dyn Shape>,
    pub material: Arc<dyn Material>,
}

pub trait Shape: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
