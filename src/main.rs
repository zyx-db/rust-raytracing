extern crate rayon;
mod extras;
use crate::extras::camera::Camera;
use crate::extras::color::Color;
use crate::extras::dielectric::Dielectric;
use crate::extras::hit::Hit;
use crate::extras::lambertian::Lambertian;
use crate::extras::metal::Metal;
use crate::extras::ray::Ray;
use crate::extras::sphere::Sphere;
use crate::extras::vec3::Vec3;
use crate::extras::world::World;
use rand::Rng;
use std::sync::Arc;
use rayon::prelude::*;

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attentuation, scattered)) = rec.mat.scatter(r, &rec) {
            return ray_color(&scattered, world, depth - 1) * attentuation;
        }
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(
        (1.0 - a) + (a * 0.5),
        (1.0 - a) + (a * 0.7),
        (1.0 - a) + (a * 1.0),
    )
}

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new((a as f64) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let r = rng.gen_range(0.0..1.0);
                let b = rng.gen_range(0.0..1.0);
                let g = rng.gen_range(0.0..1.0);
                let albedo = Color::new(r, g, b);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let r = rng.gen_range(0.4..1.0);
                let b = rng.gen_range(0.4..1.0);
                let g = rng.gen_range(0.4..1.0);
                let albedo = Color::new(r, g, b);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 25;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(lookfrom,
                          lookat,
                          vup,
                          20.0,
                          ASPECT_RATIO,
                          aperture,
                          dist_to_focus);

    // render

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", (j + 1));

        let scanline: Vec<Color> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let mut rng = rand::thread_rng();
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color
        }).collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }

    }
}
