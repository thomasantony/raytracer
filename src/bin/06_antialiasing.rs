use image::ImageBuffer;
use rand::prelude::*;
use raytracer::{color, point3, Camera, Color, Hittable, HittableList, Ray, SimpleSphere};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, 0., std::f64::INFINITY) {
        // Assume rec.normal is unit vector
        return 0.5 * (rec.normal + color(1., 1., 1.));
    }
    let unit_direction = r.direction.unit();
    // Convert y-component (-1 to 1) to blue color
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
fn main() {
    let mut rng = thread_rng();

    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio as f32) as u32;
    let samples_per_pixel = 100;
    let mut im = ImageBuffer::new(image_width, image_height);

    let camera = Camera::new();
    let mut world = HittableList::new();
    world.add(Box::new(SimpleSphere::new(point3(0., 0., -1.), 0.5)));
    world.add(Box::new(SimpleSphere::new(point3(0., -100.5, -1.), 100.)));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            let pixel = image::Rgb(pixel_color.to_rgb_scaled(samples_per_pixel));
            im.put_pixel(i, image_height - j - 1, pixel);
        }
    }
    println!("");
    im.save("./06_output.png").unwrap();
}
