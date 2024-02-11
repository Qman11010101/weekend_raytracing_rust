use s6_2_multiple_objects::{
    color::write_color, hittable::{HitRecord, Hittable, Shape}, ray::Ray, sphere::Sphere, vec3::{unit_vector, Color, Point3, Vec3}
};
use std::fs::File;
use std::io::{self, stdout, Write};

// Image Settings
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const COUNT_MAX: usize = IMAGE_HEIGHT as usize * IMAGE_WIDTH as usize;

fn ray_color(r: Ray, world: &Vec<Hittable<Sphere>>) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    if world.iter().any(|h| h.shape.hit(r, 0.0, f64::INFINITY, &mut rec)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let mut out_str = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    
    let world: Vec<Hittable<Sphere>> = vec![
        Hittable::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Hittable::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5)),
        Hittable::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let mut data_vector: Vec<String> = vec![String::from(""); COUNT_MAX];
    let mut index: usize = 0;

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("Progress: {} / {}    \r", j, IMAGE_HEIGHT);
        stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r, &world);
            data_vector[index] = write_color(pixel_color);
            index += 1;
        }
    }

    print!("\nWriting to file...");

    // Finalize
    out_str += &data_vector.join("\n");

    let mut file = File::create("6-2-multipul-objects_2.ppm").unwrap();
    file.write_fmt(format_args!("{}", out_str))?;
    println!("Done!");
    Ok(())
}
