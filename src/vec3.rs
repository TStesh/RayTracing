use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::rnd;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new() -> Self { Self([0., 0., 0.]) }

    pub fn from(x: f64, y: f64, z: f64) -> Self { Self([x, y, z]) }

    pub fn x(&self) -> f64 { return self.0[0] }
    pub fn y(&self) -> f64 { return self.0[1] }
    pub fn z(&self) -> f64 { return self.0[2] }

    // Скалярное произведение
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    // Квадрат длины вектора
    pub fn length_squared(&self) -> f64 { self.dot(self) }

    // Длина вектора
    pub fn length(&self) -> f64 { self.length_squared().powf(0.5) }

    // Нормированный вектор
    pub fn unit(&self) -> Self { *self * (1. / self.length()) }

    // Векторное произведение
    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3::from(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }

    // Случайный вектор внутри единичной сферы
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::from(
                rnd(-1., 1.),
                rnd(-1., 1.),
                rnd(-1., 1.)
            );
            if p.length_squared() < 1. { return p }
        }
    }
}

// сложение двух 3-векторов
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3::from(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) { for i in 0..3 { self.0[i] += rhs.0[i] } }
}

// Разность двух 3-векторов
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3::from(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 { self.0[i] -= rhs.0[i] }
    }
}

// Умножение 3-вектора на число
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3::from(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::from(self * rhs.0[0], self * rhs.0[1], self * rhs.0[2])
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 { self.0[i] *= rhs }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output { self * -1. }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.x())
            .field(&self.y())
            .field(&self.z())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_tests() {
        let a = Vec3::from(1., 2., 3.);
        let b = Vec3::from(3., 2., 1.);
        let c = -b;
        assert_eq!(a.dot(&b), 10.);
        assert_eq!(a.dot(&c), -10.);
    }
}