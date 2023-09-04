use log::info;
mod structs;
use structs::{Color, Ray, Vec3, World, Hit, Sphere};

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return Color::new(
            0.5 * (rec.normal.x() + 1.0),
            0.5 * (rec.normal.y() + 1.0),
            0.5 * (rec.normal.z() + 1.0)
        );
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(
        (1.0 - a) + (a * 0.5),
        (1.0 - a) + (a * 0.7),
        (1.0 - a) + (a * 1.0),
    )
}

fn main() {
    // image settings
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

    // world
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // calculate the vectors across the edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate delta from pixel to pixel
    let pixel_delta_u = &viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = &viewport_v / IMAGE_HEIGHT as f64;

    // calculate the location of upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel_top_left_location = &viewport_upper_left + &((&pixel_delta_u + &pixel_delta_v) * 0.5);

    // render

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        info!("\rScanlines remaining: {}", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let pixel_center = &(&pixel_top_left_location + &(pixel_delta_u * i as f64))
                + &(pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
            println!("{}", pixel_color);
        }
    }
}
