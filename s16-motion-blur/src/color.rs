use crate::{util::clamp, vec3::Color};

pub fn write_color(pixel_color: Color, samples_per_pixel: i32, pixel: &mut [u8; 3]) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    pixel[0] = ir;
    pixel[1] = ig;
    pixel[2] = ib;
}
