use s3_vec3::{color::write_color, vec3::Color};
use std::fs::File;
use std::io::{self, stdout, Write};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;
const COUNT_MAX: usize = IMAGE_HEIGHT as usize * IMAGE_WIDTH as usize;

fn main() -> io::Result<()> {
    let mut out_str = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut data_vector: Vec<String> = vec![String::from(""); COUNT_MAX];
    let mut index: usize = 0;

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("Progress: {} / {}    \r", j, IMAGE_HEIGHT);
        stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25 as f64,
            );
            data_vector[index] = write_color(pixel_color);
            index += 1;
        }
    }

    print!("\nWriting to file...");

    // Finalize
    out_str += &data_vector.join("\n");

    let mut file = File::create("3-vec3.ppm").unwrap();
    file.write_fmt(format_args!("{}", out_str))?;
    println!("Done!");
    Ok(())
}
