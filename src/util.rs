#![allow(dead_code)]

use rand::prelude::*;

// constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
