#![allow(dead_code)]

use crate::util::{random_double, random_range};
use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

// type aliases
pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    // constructors
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn new_empty() -> Self {
        Self { e: [0.0; 3] }
    }

    // utility functions
    pub fn x(self) -> f64 {
        self.e[0]
    }
    pub fn y(self) -> f64 {
        self.e[1]
    }
    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

// operators
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

// vec * f64
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

// f64 * vec
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        (1.0 / other) * self
    }
}

// printing
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// global utility functions
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}

pub fn random_range_vec3(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range_vec3(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
