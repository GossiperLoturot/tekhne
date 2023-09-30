use core::{
    fmt,
    iter::{Product, Sum},
    ops::*,
};

use glam::*;

/// Creates a new AABB from two points.
#[inline]
pub const fn aabb2(min: Vec2, max: Vec2) -> Aabb2 {
    Aabb2::new(min, max)
}

/// A 2-dimensional axis-aligned bounding box.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Aabb2 {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb2 {
    /// All zeroes.
    const ZERO: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::ZERO,
    };

    /// Creates a new AABB from two points.
    #[inline]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Creates a new AABB from a center point and extents.
    #[inline]
    pub fn from_center(center: Vec2, extents: Vec2) -> Self {
        Self {
            min: center - extents,
            max: center + extents,
        }
    }

    /// Returns the AABB size.
    #[inline]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    /// Returns the AABB center point.
    #[inline]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Returns the AABB extents.
    #[inline]
    pub fn extents(&self) -> Vec2 {
        (self.max - self.min) * 0.5
    }

    /// Returns the AABB volume.
    #[inline]
    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y
    }

    /// Returns a AABB with the smallest integer greater than or equal to `self`'s
    /// element as element.
    #[inline]
    pub fn ceil(&self) -> Self {
        Self {
            min: self.min.ceil(),
            max: self.max.ceil(),
        }
    }

    /// Returns a AABB with the nearest integer to `self`'s
    /// element as element.
    #[inline]
    pub fn round(&self) -> Self {
        Self {
            min: self.min.round(),
            max: self.max.round(),
        }
    }

    /// Returns a AABB with the largest integer smaller than or equal to `self`'s
    /// element as element.
    #[inline]
    pub fn floor(&self) -> Self {
        Self {
            min: self.min.floor(),
            max: self.max.floor(),
        }
    }

    /// Returns a AABB with `self`'s element integer part.
    #[inline]
    pub fn trunc(&self) -> Self {
        Self {
            min: self.min.trunc(),
            max: self.max.trunc(),
        }
    }

    /// Returns a AABB with `self`'s element fractional part.
    #[inline]
    pub fn fract(&self) -> Self {
        Self {
            min: self.min.fract(),
            max: self.max.fract(),
        }
    }

    /// Returns a AABB with `self`'s element exp.
    #[inline]
    pub fn exp(self) -> Self {
        Self {
            min: self.min.exp(),
            max: self.max.exp(),
        }
    }

    /// Returns a AABB with `self`'s element the power of n.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self {
            min: self.min.powf(n),
            max: self.max.powf(n),
        }
    }

    /// Returns a AABB with `self`'s element recip.
    #[inline]
    pub fn recip(self) -> Self {
        Self {
            min: self.min.recip(),
            max: self.max.recip(),
        }
    }

    /// Calculates the Euclidean division.
    #[inline]
    pub fn div_euclid_f32(&self, rhs: f32) -> Self {
        Self {
            min: self.min.div_euclid(vec2(rhs, rhs)),
            max: self.max.div_euclid(vec2(rhs, rhs)),
        }
    }

    /// Calculates the Euclidean division.
    #[inline]
    pub fn div_euclid_vec2(&self, rhs: Vec2) -> Self {
        Self {
            min: self.min.div_euclid(rhs),
            max: self.max.div_euclid(rhs),
        }
    }

    /// Calculates the Euclidean division.
    #[inline]
    pub fn div_euclid(&self, rhs: Aabb2) -> Self {
        Self {
            min: self.min.div_euclid(rhs.min),
            max: self.max.div_euclid(rhs.max),
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[inline]
    pub fn rem_euclid_f32(&self, rhs: f32) -> Self {
        Self {
            min: self.min.rem_euclid(vec2(rhs, rhs)),
            max: self.max.rem_euclid(vec2(rhs, rhs)),
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[inline]
    pub fn rem_euclid_vec2(&self, rhs: Vec2) -> Self {
        Self {
            min: self.min.rem_euclid(rhs),
            max: self.max.rem_euclid(rhs),
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[inline]
    pub fn rem_euclid(&self, rhs: Aabb2) -> Self {
        Self {
            min: self.min.rem_euclid(rhs.min),
            max: self.max.rem_euclid(rhs.max),
        }
    }
}

// - Aabb2
impl Neg for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn neg(self) -> Aabb2 {
        Aabb2 {
            min: self.min.neg(),
            max: self.max.neg(),
        }
    }
}

// Aabb2 + Vec2
impl Add<Vec2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn add(self, rhs: Vec2) -> Aabb2 {
        Aabb2 {
            min: self.min.add(rhs),
            max: self.max.add(rhs),
        }
    }
}

// Vec2 + Aabb2
impl Add<Aabb2> for Vec2 {
    type Output = Aabb2;
    #[inline]
    fn add(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.add(rhs.min),
            max: self.add(rhs.max),
        }
    }
}

// Aabb2 + Aabb2
impl Add<Aabb2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn add(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.min.add(rhs.min),
            max: self.max.add(rhs.max),
        }
    }
}

// Aabb2 += Vec2
impl AddAssign<Vec2> for Aabb2 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2) {
        self.min.add_assign(rhs);
        self.max.add_assign(rhs);
    }
}

// Aabb2 += Aabb2
impl AddAssign<Aabb2> for Aabb2 {
    #[inline]
    fn add_assign(&mut self, rhs: Aabb2) {
        self.min.add_assign(rhs.min);
        self.max.add_assign(rhs.max);
    }
}

// Aabb2 - Vec2
impl Sub<Vec2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn sub(self, rhs: Vec2) -> Aabb2 {
        Aabb2 {
            min: self.min.sub(rhs),
            max: self.max.sub(rhs),
        }
    }
}

// Vec2 - Aabb2
impl Sub<Aabb2> for Vec2 {
    type Output = Aabb2;
    #[inline]
    fn sub(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.sub(rhs.min),
            max: self.sub(rhs.max),
        }
    }
}

// Aabb2 - Aabb2
impl Sub<Aabb2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn sub(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.min.sub(rhs.min),
            max: self.max.sub(rhs.max),
        }
    }
}

// Aabb2 -= Vec2
impl SubAssign<Vec2> for Aabb2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        self.min.sub_assign(rhs);
        self.max.sub_assign(rhs);
    }
}

// Aabb2 -= Aabb2
impl SubAssign<Aabb2> for Aabb2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Aabb2) {
        self.min.sub_assign(rhs.min);
        self.max.sub_assign(rhs.max);
    }
}

// Aabb2 * f32
impl Mul<f32> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn mul(self, rhs: f32) -> Aabb2 {
        Aabb2 {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

// f32 * Aabb2
impl Mul<Aabb2> for f32 {
    type Output = Aabb2;
    #[inline]
    fn mul(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

// Aabb2 * Vec2
impl Mul<Vec2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Aabb2 {
        Aabb2 {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

// Vec2 * Aabb2
impl Mul<Aabb2> for Vec2 {
    type Output = Aabb2;
    #[inline]
    fn mul(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

// Aabb2 * Aabb2
impl Mul<Aabb2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn mul(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.min.mul(rhs.min),
            max: self.max.mul(rhs.max),
        }
    }
}

// Aabb2 *= f32
impl MulAssign<f32> for Aabb2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

// Aabb2 *= Vec2
impl MulAssign<Vec2> for Aabb2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec2) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

// Aabb2 *= Aabb2
impl MulAssign<Aabb2> for Aabb2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Aabb2) {
        self.min.mul_assign(rhs.min);
        self.max.mul_assign(rhs.max);
    }
}

// Aabb2 / f32
impl Div<f32> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn div(self, rhs: f32) -> Aabb2 {
        Aabb2 {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

// f32 / Aabb2
impl Div<Aabb2> for f32 {
    type Output = Aabb2;
    #[inline]
    fn div(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

// Aabb2 / Vec2
impl Div<Vec2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn div(self, rhs: Vec2) -> Aabb2 {
        Aabb2 {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

// Vec2 / Aabb2
impl Div<Aabb2> for Vec2 {
    type Output = Aabb2;
    #[inline]
    fn div(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

// Aabb2 / Aabb2
impl Div<Aabb2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn div(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.min.div(rhs.min),
            max: self.max.div(rhs.max),
        }
    }
}

// Aabb2 /= f32
impl DivAssign<f32> for Aabb2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

// Aabb2 /= Vec2
impl DivAssign<Vec2> for Aabb2 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec2) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

// Aabb2 /= Aabb2
impl DivAssign<Aabb2> for Aabb2 {
    #[inline]
    fn div_assign(&mut self, rhs: Aabb2) {
        self.min.div_assign(rhs.min);
        self.max.div_assign(rhs.max);
    }
}

// Aabb2 % f32
impl Rem<f32> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn rem(self, rhs: f32) -> Aabb2 {
        Aabb2 {
            min: self.min.rem(rhs),
            max: self.max.rem(rhs),
        }
    }
}

// f32 % Aabb2
impl Rem<Aabb2> for f32 {
    type Output = Aabb2;
    #[inline]
    fn rem(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.rem(rhs.min),
            max: self.rem(rhs.max),
        }
    }
}

// Aabb2 % Vec2
impl Rem<Vec2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn rem(self, rhs: Vec2) -> Aabb2 {
        Aabb2 {
            min: self.min.rem(rhs),
            max: self.max.rem(rhs),
        }
    }
}

// Vec2 % Aabb2
impl Rem<Aabb2> for Vec2 {
    type Output = Aabb2;
    #[inline]
    fn rem(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.rem(rhs.min),
            max: self.rem(rhs.max),
        }
    }
}

// Aabb2 % Aabb2
impl Rem<Aabb2> for Aabb2 {
    type Output = Aabb2;
    #[inline]
    fn rem(self, rhs: Aabb2) -> Aabb2 {
        Aabb2 {
            min: self.min.rem(rhs.min),
            max: self.max.rem(rhs.max),
        }
    }
}

// Aabb2 %= f32
impl RemAssign<f32> for Aabb2 {
    fn rem_assign(&mut self, rhs: f32) {
        self.min.rem_assign(rhs);
        self.max.rem_assign(rhs);
    }
}

// Aabb2 %= Vec2
impl RemAssign<Vec2> for Aabb2 {
    fn rem_assign(&mut self, rhs: Vec2) {
        self.min.rem_assign(rhs);
        self.max.rem_assign(rhs);
    }
}

// Aabb2 %= Aabb2
impl RemAssign<Aabb2> for Aabb2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Aabb2) {
        self.min.rem_assign(rhs.min);
        self.max.rem_assign(rhs.max);
    }
}

impl AsRef<[Vec2; 2]> for Aabb2 {
    #[inline]
    fn as_ref(&self) -> &[Vec2; 2] {
        unsafe { &*(self as *const Aabb2 as *const [Vec2; 2]) }
    }
}

impl AsMut<[Vec2; 2]> for Aabb2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [Vec2; 2] {
        unsafe { &mut *(self as *mut Aabb2 as *mut [Vec2; 2]) }
    }
}

impl Sum for Aabb2 {
    #[inline]
    fn sum<I: Iterator<Item = Aabb2>>(iter: I) -> Aabb2 {
        iter.fold(Aabb2::ZERO, Aabb2::add)
    }
}

impl<'a> Sum<&'a Aabb2> for Aabb2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Aabb2>>(iter: I) -> Aabb2 {
        iter.fold(Aabb2::ZERO, |a, &b| Aabb2::add(a, b))
    }
}

impl Product for Aabb2 {
    #[inline]
    fn product<I: Iterator<Item = Aabb2>>(iter: I) -> Aabb2 {
        iter.fold(Aabb2::ZERO, Aabb2::mul)
    }
}

impl<'a> Product<&'a Aabb2> for Aabb2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Aabb2>>(iter: I) -> Aabb2 {
        iter.fold(Aabb2::ZERO, |a, &b| Aabb2::mul(a, b))
    }
}

impl Index<usize> for Aabb2 {
    type Output = Vec2;
    #[inline]
    fn index(&self, index: usize) -> &Vec2 {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Aabb2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Vec2 {
        match index {
            0 => &mut self.min,
            1 => &mut self.max,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Aabb2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

impl fmt::Debug for Aabb2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Aabb2))
            .field(&self.min)
            .field(&self.max)
            .finish()
    }
}

impl From<[Vec2; 2]> for Aabb2 {
    #[inline]
    fn from(value: [Vec2; 2]) -> Aabb2 {
        Aabb2 {
            min: value[0],
            max: value[1],
        }
    }
}

impl From<Aabb2> for [Vec2; 2] {
    #[inline]
    fn from(value: Aabb2) -> [Vec2; 2] {
        [value.min, value.max]
    }
}

impl From<(Vec2, Vec2)> for Aabb2 {
    #[inline]
    fn from(value: (Vec2, Vec2)) -> Aabb2 {
        Aabb2 {
            min: value.0,
            max: value.1,
        }
    }
}

impl From<Aabb2> for (Vec2, Vec2) {
    #[inline]
    fn from(value: Aabb2) -> (Vec2, Vec2) {
        (value.min, value.max)
    }
}
