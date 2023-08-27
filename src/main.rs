use log::info;
mod structs;
use structs::{Vec3, Color};

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");
    
    for j in 0..image_height {
        info!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width as f64 - 1.0);
            let g: f64 = j as f64 / (image_height as f64 - 1.0);
            let b: f64 = 0.0;
            let color = Color::new(r, g, b);

            println!("{}", color);
        }
    }
}
