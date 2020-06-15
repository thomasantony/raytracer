use image::ImageBuffer;
use raytracer::{Color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut im = ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.);
            let g = j as f64 / (image_height as f64 - 1.);
            let b = 0.25;
            let color = Color::new(r, g, b);
            let pixel = image::Rgb(color.to_rgb());
            im.put_pixel(i, image_height - j - 1, pixel);
        }
    }
    println!("");
    im.save("./01_output.png").unwrap();
}
