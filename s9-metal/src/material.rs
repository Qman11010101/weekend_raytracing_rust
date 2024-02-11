use crate::{hittable::{HitRecord, Material}, ray::Ray, vec3::{dot, reflect, unit_vector, Color, Vec3}};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let _ = r_in;
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}