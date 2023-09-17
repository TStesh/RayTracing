use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    // инициализация
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let origin = Vec3::new();
        let horizontal = Vec3::from(viewport_width, 0., 0.);
        let vertical = Vec3::from(0., viewport_height, 0.);
        let fl = Vec3::from(0., 0., focal_length);
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - fl;
        Self { origin, horizontal, vertical, lower_left_corner }
    }
    // генерация луча
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let r = self.lower_left_corner + v * self.vertical +
            u * self.horizontal - self.origin;
        Ray::from(self.origin, r)
    }
    // цвет на экране
    pub fn write_color(&self, color: &Vec3, samples_per_pixel: u32) -> [u8; 3] {
        let (mut r, mut g, mut b) = (color.x(), color.y(), color.z());
        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let scale = 1. / samples_per_pixel as f64;
        r = (r * scale).powf(0.5);
        g = (g * scale).powf(0.5);
        b = (b * scale).powf(0.5);
        // Write the translated [0,255] value of each color component.
        [
            (256. * r.clamp(0., 0.999)) as u8,
            (256. * g.clamp(0., 0.999)) as u8,
            (256. * b.clamp(0., 0.999)) as u8
        ]
    }
}
