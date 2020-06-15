use rand::prelude::*;
use image::RgbImage;
use crate::{Camera, HittableList, color, Color, Ray, Hittable};

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

pub trait Renderer 
{
    fn render(&self,
                scene: &HittableList, 
                camera: &Camera,                   
                image_width: u32, 
                image_height: u32,
                samples_per_pixel: i32,
                max_depth: i32) -> RgbImage;
}

#[derive(Default)]
pub struct SimpleRenderer{}

impl Renderer for SimpleRenderer {
    fn render(&self,
                scene: &HittableList, 
              camera: &Camera,     
              image_width: u32, 
              image_height: u32,
              samples_per_pixel: i32,
              max_depth: i32) -> RgbImage
    {
        let mut rng = thread_rng();
        let mut im = RgbImage::new(image_width, image_height);
        for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &scene, max_depth);
            }
            let pixel = image::Rgb(pixel_color.to_rgb_scaled_gamma2(samples_per_pixel));
            im.put_pixel(i, image_height - j - 1, pixel);
        }
        }
        im
    }
}