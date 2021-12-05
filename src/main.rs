use crate::camera::*;
use crate::colour::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::sphere::*;
use crate::util::*;
use crate::vec3::*;

pub mod camera;
pub mod colour;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 128;
const MAX_DEPTH: u32 = 50;

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: u32) -> Colour {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_colour(&Ray::new(rec.point, target - rec.point), world, depth - 1);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}

fn main() {
    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let cam = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, MAX_DEPTH);
            }
            write_colour(pixel_colour, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done!");
}
