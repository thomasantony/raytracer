use rand::prelude::*;
use image::RgbImage;
use crate::{Camera, HittableList, color, Color, Ray, Hittable};
use indicatif::ParallelProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};

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
                scene: HittableList, 
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
                scene: HittableList, 
              camera: &Camera,     
              image_width: u32, 
              image_height: u32,
              samples_per_pixel: i32,
              max_depth: i32) -> RgbImage
    {
        let mut rng = thread_rng();
        let mut im = RgbImage::new(image_width, image_height);
        let pb = indicatif::ProgressBar::new((image_width*image_height).into());
        pb.set_style(ProgressStyle::default_bar()
                     .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"));
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
            pb.inc(1);
        }
        }
        pb.finish_with_message("done");
        im
    }
}

use std::sync::{Arc};
use std::sync::mpsc;
use rayon::prelude::*;
use std::thread;

#[derive(Default)]
pub struct RayonRenderer{}

impl Renderer for RayonRenderer {
    fn render(&self,
              scene: HittableList, 
              camera: &Camera,     
              image_width: u32, 
              image_height: u32,
              samples_per_pixel: i32,
              max_depth: i32) -> RgbImage
    {
        let (tx, rx) = mpsc::channel();
        // Collects and writes pixels to image buffer
        let writer_thread = thread::spawn(move || {
            let mut im = RgbImage::new(image_width, image_height);
            loop {
                // Type: Option<(u32, u32, Color)>
                let next_pixel = rx.recv().unwrap();
                if let Some((x, y, p)) = next_pixel
                {
                    im.put_pixel(x, y, p)
                }else{
                    break;
                }
            }
            im
        });

        let scene = Arc::new(scene);
        // let mut im = RgbImage::new(image_width, image_height);
        let n_pixels = image_width * image_height;
        let pb = indicatif::ProgressBar::new(n_pixels.into());
        pb.set_style(ProgressStyle::default_bar()
                     .template("{spinner:.green} [{bar:40.cyan/blue}] {percent}% ({elapsed_precise}/{eta_precise})"));
        let tx2 = tx.clone();
        // Column major form
        // pixel_idx = i * image_height + j
        (0..n_pixels)
            .into_par_iter()
            .progress_with(pb)
            .map_with(scene, |scene, pixel_idx| {
                let mut rng = thread_rng();
                let j = image_height - (pixel_idx % image_height) - 1;
                let i = (pixel_idx as f64 / image_height as f64).floor() as u32;
                let mut pixel_color = color(0., 0., 0.);
                for _ in 0..samples_per_pixel {
                    let u = ((i as f64) + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                    let v = ((j as f64) + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &scene, max_depth);
                }
                let pixel = image::Rgb(pixel_color.to_rgb_scaled_gamma2(samples_per_pixel));
                // im.put_pixel(i, image_height - j - 1, pixel);
                return Some((i, image_height - j - 1, pixel));
            }).try_for_each_with(tx, |tx, item| {
                tx.send(item)
            }).unwrap();
        tx2.send(None).unwrap();

        let im = writer_thread.join().unwrap();
        im
    }
}