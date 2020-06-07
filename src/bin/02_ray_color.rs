use image::ImageBuffer;
use raytracer::{Color, Point3, Ray, Vec3};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio as f32) as u32;

    let mut im = ImageBuffer::new(image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    println!(
        "h: {:?}\nv: {:?}\nll: {:?}",
        horizontal, vertical, lower_left_corner
    );
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            let pixel = image::Rgb(pixel_color.to_rgb());
            im.put_pixel(i, image_height - j - 1, pixel);
        }
    }
    println!("");
    im.save("./02_output.png").unwrap();
}
