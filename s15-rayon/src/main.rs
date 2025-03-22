use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use s15_exp2::{
    camera::Camera,
    color::write_color,
    hittable::{HitRecord, Hittable},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    util::{random_f64, random_f64_range},
    vec3::{unit_vector, Color, Point3, Vec3},
};
use std::fs::File;
use std::io::{self, Write};
use std::sync::Arc;

// Image Settings
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 640;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

const COUNT_MAX: usize = IMAGE_HEIGHT as usize * IMAGE_WIDTH as usize;

fn ray_color(r: Ray, world: &Vec<Hittable>, depth: i32) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        t: 0.0,
        front_face: false,
    };

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut closest_so_far = f64::INFINITY;
    let mut hit_anything = false;
    let mut temp_rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        t: 0.0,
        front_face: false,
    };

    for hittable in world.iter() {
        temp_rec.material = Arc::clone(&hittable.material);
        if hittable.shape.hit(r, 0.001, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t.clone();
            rec.p = temp_rec.p.clone();
            rec.normal = temp_rec.normal.clone();
            rec.material = Arc::clone(&temp_rec.material);
            rec.t = temp_rec.t.clone();
            rec.front_face = temp_rec.front_face.clone();
        }
    }

    if hit_anything {
        let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);

        if Arc::clone(&rec.material).scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> Vec<Hittable> {
    let mut world: Vec<Hittable> = Vec::new();

    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    world.push(Hittable::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0),
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.push(Hittable::new(Sphere::new(center, 0.2), sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.push(Hittable::new(Sphere::new(center, 0.2), sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Hittable::new(Sphere::new(center, 0.2), sphere_material));
                }
            }
        }
    }

    world.push(Hittable::new(
        Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0),
        Dielectric::new(1.5),
    ));
    world.push(Hittable::new(
        Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0),
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Hittable::new(
        Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0),
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

fn main() -> io::Result<()> {
    let mut out_str = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut data_array_raw: Vec<[u8; 3]> = vec![[0; 3]; COUNT_MAX];

    data_array_raw
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, pixel)| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let x = idx % IMAGE_WIDTH as usize;
            let y = IMAGE_HEIGHT - (idx / IMAGE_WIDTH as usize) as i32;

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL, pixel);
        });

    print!("\nWriting to file...");

    // Finalize
    let result = data_array_raw
        .iter()
        .map(|x| format!("{} {} {}\n", x[0], x[1], x[2]))
        .collect::<String>();
    out_str.push_str(&result);

    let mut file = File::create("15-rayon.ppm").unwrap();
    file.write_fmt(format_args!("{}", out_str))?;
    println!("Done!");
    Ok(())
}
