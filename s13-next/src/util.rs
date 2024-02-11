use rand::distributions::{Uniform, Distribution};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0.0..1.0);
    uniform.sample(&mut rng)
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(min..max);
    uniform.sample(&mut rng)
}