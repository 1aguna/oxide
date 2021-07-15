use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign};
use std::ops;

use crate::linalg::point::Point;

/// Vector is a standard 3 component vector
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// Initialize the vector and set values for x, y, z
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
    /// Initialize the vector with the same value of x, y, z
    pub fn broadcast(x: f32) -> Vector {
        Vector { x: x, y: x, z: x }
    }
    /// Compute the squared length of the vector
    pub fn length_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Compute the length of the vector
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_sqr())
    }
    /// Get a normalized copy of this vector
    pub fn normalized(&self) -> Vector {
        let len = self.length();
        Vector { x: self.x / len, y: self.y / len, z: self.z / len }
    }
    /// Normalize the vector in place
    pub fn normalize(&mut self)  {
        let len = self.length();
        *self /= len;
    }

    pub fn dot(&self, b: &Vector) -> f32 {
        self.x * b.x +
        self.y * b.y +
        self.z * b.z
    }

    pub fn cross(&self, b: &Vector) -> Vector {
        Vector::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
}

// Binary Operators
impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector {
    Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
});

impl_op_ex!(- |a: &Vector, b: &Vector| -> Vector {
    Vector {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z
    }
});

impl_op_ex_commutative!(* |a: &Vector, scalar: f32 | -> Vector {
    Vector {
        x: a.x * scalar,
        y: a.y * scalar,
        z: a.z * scalar,
    }
});

impl_op_ex!(/ |a: &Vector, scalar: f32| -> Vector {
    Vector {
        x: a.x / scalar,
        y: a.y / scalar,
        z: a.z / scalar,
    }
});

// Unary operators
impl_op_ex!(- |a: &Vector| -> Vector {
    Vector {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});

// Assignment operators
impl_op_ex!(+= |a: &mut Vector, b: &Vector| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

// impl AddAssign<f32> for Vector {
//     fn add_assign(&mut self, rhs: f32) {
//         self.x += rhs;
//         self.y += rhs;
//         self.z += rhs;
//     }
// }
//
// impl MulAssign<f32> for Vector {
//     fn mul_assign(&mut self, rhs: f32) {
//         self.x *= rhs;
//         self.y *= rhs;
//         self.z *= rhs;
//     }
// }
//
// impl DivAssign<f32> for Vector {
//     fn div_assign(&mut self, rhs: f32) {
//         self.x /= rhs;
//         self.y /= rhs;
//         self.z /= rhs;
//     }
// }

impl_op_ex!(-= |a: &mut Vector, b: &Vector| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op!(/= |a: &mut Vector, b: f32| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
});

impl_op_ex!(*= |a: &mut Vector, b: f32| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});


impl Index<usize> for Vector {
    type Output = f32;
    /// Access the vector by index
    ///
    /// - 0 = x
    /// - 1 = y
    /// - 2 = z
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index into vector"),
        }
    }
}

impl IndexMut<usize> for Vector {
    /// Access the vector by index
    ///
    /// - 0 = x
    /// - 1 = y
    /// - 2 = z
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index into vector"),
        }
    }
}

pub fn dot(a: &Vector, b: &Vector) -> f32 {
    a.dot(b)
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    a.cross(b)
}