use crate::vec3::Color;

pub fn write_color(pixel_color: Color) -> String {
    format!(
        "{} {} {}",
        (255.999 as f64 * pixel_color.x()) as u8,
        (255.999 as f64 * pixel_color.y()) as u8,
        (255.999 as f64 * pixel_color.z()) as u8
    )
}
