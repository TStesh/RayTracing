#![allow(unused)]
pub mod vec3;
pub mod camera;
pub mod ray;
pub mod hitrecord;
pub mod sphere;
pub mod raster;

use std::rc::Rc;
use image::{ImageBuffer, Rgb};
use rand::Rng;

use crate::camera::Camera;
use crate::hitrecord::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub const PI: f64 = std::f64::consts::PI;

pub const INF: f64 = f64::INFINITY;

// Image
pub const ASPECT_RATIO: f64 = 16. / 9.;
pub const IMAGE_WIDTH: u32 = 600;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 100;
pub const MAX_DEPTH: u32 = 50;

pub fn degrees_to_radians(degrees: f64) -> f64 { degrees * PI / 180. }

pub fn rnd(x: f64, y: f64) -> f64 { rand::thread_rng().gen_range(x..y) }

pub fn render(filename: &str, png_mode: bool, world: &HittableList<Sphere>) {
    // Path
    let mut file = filename.to_string();
    if png_mode {
        file.push_str(".png")
    } else {
        file.push_str(".ppm")
    };

    // Camera
    let cam = Camera::new(ASPECT_RATIO);

    // Render
    let mut img = ImageBuffer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT
    );
    // let mut ps = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut b = vec![];
    for x in format!("P6 {} {} 255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes() {
        b.push(*x);
    }
    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let mut c = Vec3::new();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (j as f64 + rnd(-1., 1.)) / (IMAGE_WIDTH as f64 - 1.);
                let v = (i as f64 + rnd(-1., 1.)) / (IMAGE_HEIGHT as f64 - 1.);
                c += cam.get_ray(u, v).ray_color(&world, MAX_DEPTH);
            }
            let a = cam.write_color(&c, SAMPLES_PER_PIXEL);
            if png_mode {
                img.put_pixel(j, IMAGE_HEIGHT - i - 1, Rgb::from(a));
            } else {
                //for k in 0..2 { ps.push_str(format!("{} ", a[k] as u32).as_str()) }
                //ps.push_str(a[2].to_string().as_str());
                //ps.push_str("\n");
                for i in 0..2 { b.push(a[i]); }
            }
        }
    }
    //Save image
    if png_mode {
        img.save(file).unwrap();
    } else {
        std::fs::write(file, &b).unwrap();
    }
}