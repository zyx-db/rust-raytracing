extern crate rayon;
extern crate gif;
mod utils;
use utils::camera::Camera;
use utils::color::Color;
use utils::dielectric::Dielectric;
use utils::lambertian::Lambertian;
use utils::metal::Metal;
use utils::sphere::Sphere;
use utils::vec3::Vec3;
use rand::Rng;
use utils::aabb::Tree;
use std::fs::File;
use std::sync::Arc;
use gif::Frame;

fn random_scene() -> Tree {
    let mut rng = rand::thread_rng();
    let mut world = Tree::new(1);

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat, Vec3::new(0.0 ,0.0, 0.0));

    world.push(Box::new(ground_sphere));

    // for a in 0..5 {
    //     for b in 0..5 {
    //         let choose_mat: f64 = rng.gen();
    //         let center = Vec3::new((a as f64) + rng.gen_range(0.0..0.9),
    //                                  0.2,
    //                                  (b as f64) + rng.gen_range(0.0..0.9));
    //         let movement = Vec3::new((a as f64) + rng.gen_range(0.0..0.9),
    //                                  0.2,
    //                                  (b as f64) + rng.gen_range(0.0..0.9));

    //         if choose_mat < 0.8 {
    //             // Diffuse
    //             let r = rng.gen_range(0.0..1.0);
    //             let b = rng.gen_range(0.0..1.0);
    //             let g = rng.gen_range(0.0..1.0);
    //             let albedo = Color::new(r, g, b);
    //             let sphere_mat = Arc::new(Lambertian::new(albedo));
    //             let sphere = Sphere::new(center, 0.2, sphere_mat, movement);

    //             world.push(Box::new(sphere));
    //         } else if choose_mat < 0.95 {
    //             // Metal
    //             let r = rng.gen_range(0.4..1.0);
    //             let b = rng.gen_range(0.4..1.0);
    //             let g = rng.gen_range(0.4..1.0);
    //             let albedo = Color::new(r, g, b);
    //             let fuzz = rng.gen_range(0.0..0.5);
    //             let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
    //             let sphere = Sphere::new(center, 0.2, sphere_mat, movement);

    //             world.push(Box::new(sphere));
    //         } else {
    //             // Glass
    //             let sphere_mat = Arc::new(Dielectric::new(1.5));
    //             let sphere = Sphere::new(center, 0.2, sphere_mat, movement);

    //             world.push(Box::new(sphere));
    //         }
    //     }
    // }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1, Vec3::new(0.0, 0.1, 0.0));
    let sphere2 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2, Vec3::new(0.0, 0.1, 0.0));
    let sphere3 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3, Vec3::new(0.0, 0.1, 0.0));

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 600;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 50;
    const MAX_DEPTH: u64 = 20;

    // World
    let mut world = random_scene();

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
                          dist_to_focus,
                          IMAGE_HEIGHT,
                          IMAGE_WIDTH,
                          SAMPLES_PER_PIXEL,
                          MAX_DEPTH);

    let frames = 10;
    let time_delta = 0.1;

    // render
    let mut file = File::create("results/test.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut file, IMAGE_WIDTH as u16, IMAGE_HEIGHT as u16, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    let mut i = 0;
    for _ in 0..frames{
        i += 1;
        let pixels = cam.render(&world);
        let frame = Frame::from_rgb(IMAGE_WIDTH as u16, IMAGE_HEIGHT as u16, &pixels);
        encoder.write_frame(&frame).unwrap();
        eprint!("\rfinished frame {}          ", i);
        if i == frames{
            break;
        }
        world = world.step_frame(time_delta);
        eprint!("\rfinished moving {}         ", i);
    }
}
