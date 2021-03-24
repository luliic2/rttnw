#![allow(dead_code)]

use rand::Rng;

use std::fmt;
use std::marker::PhantomData;

/// Trait that guards the possible types for Vec3f<T>
pub trait Phantom {}
pub trait PhantomColor: Phantom {}
pub trait PhantomPosition: Phantom {}
type Precision = f64;

/// Struct that defines a vector of size 3
/// The type parameter it's to improve type safety
/// by not allowing operations in two conceptually different vectors
///
/// ```
/// let v1: Vec3f<Color> = (1.0, 0.0, 0.0).into();
/// let v2: Vec3f<Color> = (0.0, 2.0, 0.0).into();
/// let v3: Vec3f<Position> = (0.0, 0.0, 3.0).into();
/// // This is fine
/// let v4 = v1 + v2;
/// // This is not fine
/// // let v4 = v1 + v3;
/// ```
pub struct Vec3f<T> {
    items: [Precision; 3],
    _phantom: PhantomData<T>,
}

// Must be implemented manually because PhantomData<T> does not implement them
impl<T: Phantom> Copy for Vec3f<T> {}
impl<T: Phantom> Clone for Vec3f<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Vec3f<T>
where
    T: Phantom,
{
    pub fn scaled(items: &[u8], scale: Precision) -> Self {
        assert!(items.len() >= 3);
        Self::new(
            items[0] as Precision * scale,
            items[1] as Precision * scale,
            items[2] as Precision * scale,
        )
    }
    pub fn new(x: Precision, y: Precision, z: Precision) -> Self {
        Self {
            items: [x, y, z],
            _phantom: PhantomData::<T>,
        }
    }
    pub fn random(range: std::ops::Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(range.clone());
        let y = rng.gen_range(range.clone());
        let z = rng.gen_range(range);
        Self::new(x, y, z)
    }

    pub fn x(&self) -> Precision {
        self.items[0]
    }
    pub fn y(&self) -> Precision {
        self.items[1]
    }
    pub fn z(&self) -> Precision {
        self.items[2]
    }

    /// Dot product of two vectors
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    /// Cross product of two vectors
    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            -(self.x() * rhs.z() - self.z() * rhs.x()),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.x().powf(2.0) + self.y().powf(2.0) + self.z().powf(2.0)).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.x().powf(2.0) + self.y().powf(2.0) + self.z().powf(2.0)
    }

    pub fn unit(&self) -> Self {
        let k = 1.0 / self.magnitude();
        *self * k
    }

    pub fn repeat(x: f64) -> Self {
        Self::new(x, x, x)
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(Precision) -> Precision,
    {
        Self::new(f(self.x()), f(self.y()), f(self.z()))
    }
    pub fn reflect(&self, n: Vec3f<T>) -> Vec3f<T> {
        *self - 2.0 * self.dot(n) * n
    }
    pub fn refract(&self, n: Vec3f<T>, ni_over_nt: f64) -> Option<Vec3f<T>> {
        let unit = self.unit();
        let dt = unit.dot(n);
        let discriminant = 1.0 - ni_over_nt.powf(2.0) * (1.0 - dt.powf(2.0));
        if discriminant > 0.0 {
            let result = ni_over_nt * (unit - n * dt) - n * discriminant.sqrt();
            Some(result)
        } else {
            None
        }
    }
    pub fn at<C>(&self, x: C) -> Precision
    where
        C: Index,
    {
        self.items[x.into()]
    }

    pub fn near_zero(&self) -> bool {
        let almost_zero = 1e-8;
        self.x().abs() < almost_zero && self.y().abs() < almost_zero && self.z().abs() < almost_zero
    }

    pub fn with_dimension<C>(mut self, dimension: C, value: Precision) -> Self
    where
        C: Into<usize>,
    {
        self.items[dimension.into()] = value;
        self
    }
}

impl<T: Phantom> Default for Vec3f<T> {
    fn default() -> Self {
        Self::repeat(0.0)
    }
}

impl Vec3f<Position> {
    pub fn random_in_unit_space() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            // Random point where (x, y, z) belong to -1..1
            let vector = 2.0 * Vec3f::new(rng.gen(), rng.gen(), rng.gen()) - Vec3f::repeat(1.0);
            if vector.squared_length() < 1.0 {
                return vector;
            }
        }
    }
}
impl<T> Vec3f<T>
where
    T: PhantomColor,
{
    pub fn r(&self) -> f64 {
        self.x()
    }
    pub fn g(&self) -> f64 {
        self.y()
    }
    pub fn b(&self) -> f64 {
        self.z()
    }
}

// Possible types of a Vec3f
pub struct Color;
pub struct Position;
impl Phantom for Color {}
impl Phantom for Position {}
impl PhantomColor for Color {}
impl PhantomPosition for Position {}

// Basic operations
impl<T: Phantom> From<(Precision, Precision, Precision)> for Vec3f<T> {
    fn from(x: (Precision, Precision, Precision)) -> Self {
        Self::new(x.0, x.1, x.2)
    }
}
impl<T: Phantom> From<&[Precision]> for Vec3f<T> {
    fn from(items: &[f64]) -> Self {
        assert!(items.len() >= 3);
        Self::new(items[0], items[1], items[2])
    }
}
impl<T: Phantom> std::ops::Add for Vec3f<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}
impl<T: Phantom> std::ops::Sub for Vec3f<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}
/// Element-wise multiplication of two vectors.
impl<T: Phantom> std::ops::Mul for Vec3f<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}
impl<T: Phantom> std::ops::Mul<f64> for Vec3f<T> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl<T: Phantom> std::ops::Mul<Vec3f<T>> for f64 {
    type Output = Vec3f<T>;
    fn mul(self, rhs: Vec3f<T>) -> Self::Output {
        Self::Output::new(rhs.x() * self, rhs.y() * self, rhs.z() * self)
    }
}

impl<T: Phantom> std::ops::Div for Vec3f<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}
impl<T: Phantom> std::ops::Div<f64> for Vec3f<T> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl<T: Phantom> fmt::Display for Vec3f<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
impl<T: Phantom> std::ops::Neg for Vec3f<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}
impl<T: Phantom> std::iter::Sum for Vec3f<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::repeat(0.0), std::ops::Add::add)
    }
}

/// Trait for items that can index `Vec3f`.
/// Poor man `std::slice::SliceIndex` because its unsafe & unstable.
pub trait Index: Into<usize> {}
impl Index for usize {}
impl Index for Coordinate {}

// Before the trait bound was `where C: Into<usize>`,
// but then the compiler asked for implicit type for integers.
// What happened is:
// ```
// let v = Vec3f::default();
// v[0]; // Compiler error: i32 does not implemented Into<usize>
// v[0_usize]; // Forced to do this
// ...
// ```
impl<T: Phantom, C> std::ops::Index<C> for Vec3f<T> where C: Index {
    type Output = Precision;

    fn index(&self, index: C) -> &Self::Output {
        &self.items[index.into()]
    }
}

impl<T: Phantom, C> std::ops::IndexMut<C> for Vec3f<T> where C: Index {
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        &mut self.items[index.into()]
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Coordinate {
    X,
    Y,
    Z,
}

impl Into<usize> for Coordinate {
    fn into(self) -> usize {
        self as usize
    }
}
