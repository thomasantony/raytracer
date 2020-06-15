use std::rc::Rc;
use image::ImageBuffer;
use rand::prelude::*;
use raytracer::{color, point3, Vec3, Camera, Color, Hittable, HittableList, Ray, Sphere};
use raytracer::materials::{Lambertian, Metal, Dielectric};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0
    {
        return color(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) 
    {
        if let Some((scattered_ray, attenuation)) = rec.scatter(r)
        {
            return attenuation * ray_color(&scattered_ray, world, depth -1);
        }
        return color(0., 0., 0.);
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
    let max_depth = 50;

    let mut im = ImageBuffer::new(image_width, image_height);

    let lookfrom = point3(3., 3., 2.);
    let lookat = point3(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;

    let camera = Camera::new_with_depth_of_field(lookfrom, lookat, vup, 20., aspect_ratio, aperture, dist_to_focus);

    let mut world = HittableList::new();
    
    let material_1 = Rc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let material_2 = Rc::new(Lambertian::new(color(0.8, 0.8, 0.0)));

    let metal_1 = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 0.5));
    // let metal_2 = Rc::new(Metal::new(color(0.8, 0.8, 0.8), 0.3));

    let dielectric_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(point3(0., 0., -1.), 0.5, material_1.clone())));
    world.add(Box::new(Sphere::new(point3(0., -100.5, -1.), 100., material_2.clone())));

    world.add(Box::new(Sphere::new(point3(1.,0.,-1.), 0.5, metal_1.clone())));
    // world.add(Box::new(Sphere::new(point3(-1.,0.,-1.), 0.5, metal_2.clone())));
    world.add(Box::new(Sphere::new(point3(-1.,0.,-1.), 0.5, dielectric_1.clone())));
    world.add(Box::new(Sphere::new(point3(-1.,0.,-1.), -0.45, dielectric_1.clone())));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            let pixel = image::Rgb(pixel_color.to_rgb_scaled_gamma2(samples_per_pixel));
            im.put_pixel(i, image_height - j - 1, pixel);
        }
    }
    println!("");
    im.save("./12_output.png").unwrap();
}
