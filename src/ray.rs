use crate::hitrecord::{HitRecord, Hittable, HittableList};
use crate::INF;
use crate::vec3::Vec3;

pub struct Ray { pub(crate) orig: Vec3, pub(crate) dir: Vec3 }

impl Ray {
    // origin_point - точка-источник луча
    // direction - направляющий вектор луча
    pub fn from(origin_point: Vec3, direction: Vec3) -> Self {
        Self { orig: origin_point, dir: direction }
    }

    // Точка луча A(t) = origin_point + t * direction
    pub fn at(&self, t: f64) -> Vec3 { self.orig + t * self.dir }

    // цвет точки луча
    pub fn ray_color<T: Hittable>(&self, world: &HittableList<T>, depth: u32) -> Vec3 {
        let mut rec = HitRecord::new();
        if depth <= 0 { return Vec3::new() }
        if world.hit(&self, 0.001, INF, &mut rec) {
            // цвет точки пересечения
            let target = rec.normal + Vec3::random_in_unit_sphere();
            let ray_target = Ray::from(rec.p, target);
            return 0.5 * ray_target.ray_color(world, depth - 1)
        }
        // если не было пересечений, то возвращаем цвет фона
        // цвет фона формируется как градиентная заливка от белого к голубому по Y-координате
        let t = 0.5 * (self.dir.unit().y() + 1.);
        (1. - t) * Vec3::from(1., 1., 1.) + t * Vec3::from(0.5, 0.7, 1.)
    }
}