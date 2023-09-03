use log::info;
mod structs;
use structs::{Color, Ray, Vec3};

fn hit_sphere(center: &Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new((n.x() + 1.0) * 0.5, (n.y() + 1.0) * 0.5, (n.z() + 1.0) * 0.5)
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // calculate image height
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // calculate the vectors across the edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate delta from pixel to pixel
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // calculate the location of upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel_top_left_location = &viewport_upper_left + &((&pixel_delta_u + &pixel_delta_v) * 0.5);

    // render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        info!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center = &(&pixel_top_left_location + &(pixel_delta_u * i as f64))
                + &(pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            println!("{}", pixel_color);
        }
    }
}
