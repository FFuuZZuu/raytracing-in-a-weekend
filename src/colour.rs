use crate::util::clamp;
use crate::vec3::Colour;

pub fn write_colour(pixel_colour: Colour, samples_per_pixel: u32) {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= (scale * r).sqrt();
    g *= (scale * g).sqrt();
    b *= (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32
    );
}
