use image::{ImageBuffer};
use raytracer::Vec3;

fn main() {
    let a = Vec3::default();
    let image_width = 256;
    let image_height = 256;

    let mut im = ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev()
    {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width
        {
            let r = i as f64 / (image_width as f64 - 1.);
            let g = j as f64 / (image_height as f64 - 1.);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            let pixel = image::Rgb([ir, ig, ib]);
            im.put_pixel(i, image_height-j-1, pixel);
        }
    }
    println!("");
    im.save("./output.png").unwrap();
}


