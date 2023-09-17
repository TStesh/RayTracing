use std::rc::Rc;
use crate::ray::Ray;
use crate::vec3::Vec3;

// Структура для хранения параметров пересечения
// p - точка пересечения
// normal - нормаль к поверхности в точке пересечения
// t - параметр точки пересечения
// front_face - нормаль смотрит на нас (true) или от нас (false)
pub struct HitRecord { pub(crate) p: Vec3, pub(crate) normal: Vec3, pub(crate) t: f64, front_face: bool }

impl HitRecord {
    pub fn new() -> Self { Self {
        p: Vec3::new(), normal: Vec3::new(), t: 0., front_face: false
    } }
    // функция определения видимой поверхности
    // Если нормаль всегда внешняя, то определить с какой стороны объекта подходит луч тривиально -
    // если b направление луча,а N - нормаль, то (b, N) > 0 если угол между b и N острый, то есть
    // b и N смотрят в одну сторону и значит луч "выходит" из объекта, то есть изнутри пересекает
    // объект. Если же (b, N) < 0, то угол между векторами тупой, луч и нормаль антинаправлены,
    // поэтому луч "входит" в объект, то есть пересекает его извне.
    // front_face = true, если луч "входит" в объект => (b, N) < 0, иначе false
    // normal = нормаль с направлением по лучу
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

// Трейт для пересечений, определяет поведение в случае пересечения с лучом
pub trait Hittable {
    // t_min, t_max - допустимый интервал возможных значений параметра пересечения
    // по умолчанию совпадает со всей сценой: t_min = 1, t_max = +inf
    // rec - структура, в которой записаны данные о ближайшей точке пересечения
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

// Структура для хранения списка пересекаемых объектов
// Разумеется, она ограничена объектами, реализующими трейт Hittable
pub struct HittableList<T: Hittable> { objects: Vec<Rc<T>> }

impl<T> HittableList<T> where T: Hittable {
    pub fn new(obj: Rc<T>) -> Self {
        let mut v: Vec<Rc<T>> = Vec::new();
        v.push(obj);
        Self { objects: v }
    }
    pub fn add(&mut self, obj: Rc<T>) { self.objects.push(obj) }
    fn clear(&mut self) { self.objects.clear() }
}

// Реализация трейта Hittable для структуры HittableList
// Суть простая - нужно пробежаться по всем объектам структуры
// для каждого объекта определить, пересекает ли его луч
// если пересекает, то
// Определить для каждого пересекаемого объекта ближайшую к камере точку пересечения
// Записать
impl<T> Hittable for HittableList<T> where T: Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            let mut temp_rec = HitRecord::new();
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
