use crate::hitrecord::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere { center: Vec3, radius: f64 }

impl Sphere {
    pub fn from(center: Vec3, radius: f64) -> Self { Self { center, radius } }
}

// Пересечение со сферой
// находим точку пересчения и записываем параметры пересечения в HitRecord
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = r.dir.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. { return false }
        let sqrtd = discriminant.powf(0.5);
        // Найдем ближайший корень, который попадает в допустимый интервал [t_min; t_max]
        let root = -(sqrtd + half_b) / a;
        if root < t_min || root > t_max {
            // проверяем второй корень
            let root = (sqrtd - half_b) / a;
            if root < t_min || root > t_max { return false }
        }
        // фиксируем данные о пересчении
        // фактор пересечения
        rec.t = root;
        // точка луча
        rec.p = r.at(root);
        // нормаль из центра сферы в точку пересечения
        let outward_normal = (rec.p - self.center) * (1. / self.radius);
        rec.set_face_normal(r, &outward_normal);
        true
    }
}
