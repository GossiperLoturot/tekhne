use core::{iter, mem, ops::*};

use glam::*;

use super::*;

/// Creates a new AABB from two points.
#[inline]
pub const fn iaabb2(min: IVec2, max: IVec2) -> IAabb2 {
    IAabb2::new(min, max)
}

/// A 2-dimensional axis-aligned bounding box.
#[repr(C, align(16))]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct IAabb2 {
    pub min: IVec2,
    pub max: IVec2,
}

impl IAabb2 {
    /// All zeroes.
    const ZERO: Self = Self {
        min: IVec2::ZERO,
        max: IVec2::ZERO,
    };

    /// Creates a new AABB from two points.
    #[inline]
    pub const fn new(min: IVec2, max: IVec2) -> Self {
        Self { min, max }
    }

    /// Creates a new AABB from a center point and extents.
    #[inline]
    pub fn from_center(center: IVec2, extents: IVec2) -> Self {
        Self {
            min: center - extents,
            max: center + extents,
        }
    }

    /// Returns the AABB size.
    #[inline]
    pub fn size(&self) -> IVec2 {
        self.max - self.min
    }

    /// Returns the AABB center point.
    #[inline]
    pub fn center(&self) -> IVec2 {
        (self.min + self.max) >> 1
    }

    /// Returns the AABB extents.
    #[inline]
    pub fn extents(&self) -> IVec2 {
        (self.max - self.min) >> 1
    }

    /// Returns the AABB volume.
    #[inline]
    pub fn volume(&self) -> i32 {
        let size = self.size();
        size.x * size.y
    }

    /// Returns the AABB with extended size.
    #[inline]
    pub fn extends(self, size: i32) -> Self {
        Self {
            min: self.min - IVec2::splat(size),
            max: self.max + IVec2::splat(size),
        }
    }

    /// Calculates the Euclidean division.
    #[inline]
    pub fn div_euclid_i32(self, rhs: i32) -> Self {
        Self {
            min: self.min.div_euclid(ivec2(rhs, rhs)),
            max: self.max.div_euclid(ivec2(rhs, rhs)),
        }
    }

    /// Calculates the Euclidean division.
    #[inline]
    pub fn div_euclid_ivec2(self, rhs: IVec2) -> Self {
        Self {
            min: self.min.div_euclid(rhs),
            max: self.max.div_euclid(rhs),
        }
    }

    /// Calculates the Euclidean division.
    #[inline]
    pub fn div_euclid(self, rhs: IAabb2) -> Self {
        Self {
            min: self.min.div_euclid(rhs.min),
            max: self.max.div_euclid(rhs.max),
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[inline]
    pub fn rem_euclid_i32(self, rhs: i32) -> Self {
        Self {
            min: self.min.rem_euclid(ivec2(rhs, rhs)),
            max: self.max.rem_euclid(ivec2(rhs, rhs)),
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[inline]
    pub fn rem_euclid_ivec2(self, rhs: IVec2) -> Self {
        Self {
            min: self.min.rem_euclid(rhs),
            max: self.max.rem_euclid(rhs),
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[inline]
    pub fn rem_euclid(self, rhs: IAabb2) -> Self {
        Self {
            min: self.min.rem_euclid(rhs.min),
            max: self.max.rem_euclid(rhs.max),
        }
    }

    /// Returns whether if `self` contains `rhs`.
    #[inline]
    pub fn contains_point(&self, rhs: IVec2) -> bool {
        self.min.x <= rhs.x && self.min.y <= rhs.y && rhs.x < self.max.x && rhs.y < self.max.y
    }

    /// Returns whether if `self` intersects `rhs`.
    #[inline]
    pub fn contains_rect(&self, rhs: IAabb2) -> bool {
        self.min.x <= rhs.min.x
            && self.min.y <= rhs.min.y
            && rhs.max.x <= self.max.x
            && rhs.max.y <= self.max.y
    }

    /// Returns whether if `self` intersects `rhs`.
    #[inline]
    pub fn intersects(&self, rhs: IAabb2) -> bool {
        self.min.x < rhs.max.x
            && self.min.y < rhs.max.y
            && rhs.min.x < self.max.x
            && rhs.min.y < self.max.y
    }

    /// Returns an iterator visiting all points contained by `self` area.
    #[inline]
    pub fn into_iter_points(self) -> IterPoint {
        IterPoint {
            rect: self,
            point: self.min,
        }
    }

    /// Casts into `Aabb2`.
    #[inline]
    pub fn as_aabb2(&self) -> Aabb2 {
        Aabb2 {
            min: self.min.as_vec2(),
            max: self.max.as_vec2(),
        }
    }
}

// - IAabb2
impl Neg for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn neg(self) -> IAabb2 {
        IAabb2 {
            min: self.min.neg(),
            max: self.max.neg(),
        }
    }
}

// IAabb2 + IVec2
impl Add<IVec2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn add(self, rhs: IVec2) -> IAabb2 {
        IAabb2 {
            min: self.min.add(rhs),
            max: self.max.add(rhs),
        }
    }
}

// IVec2 + IAabb2
impl Add<IAabb2> for IVec2 {
    type Output = IAabb2;
    #[inline]
    fn add(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.add(rhs.min),
            max: self.add(rhs.max),
        }
    }
}

// IAabb2 + IAabb2
impl Add<IAabb2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn add(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.min.add(rhs.min),
            max: self.max.add(rhs.max),
        }
    }
}

// IAabb2 += IVec2
impl AddAssign<IVec2> for IAabb2 {
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        self.min.add_assign(rhs);
        self.max.add_assign(rhs);
    }
}

// IAabb2 += IAabb2
impl AddAssign<IAabb2> for IAabb2 {
    #[inline]
    fn add_assign(&mut self, rhs: IAabb2) {
        self.min.add_assign(rhs.min);
        self.max.add_assign(rhs.max);
    }
}

// IAabb2 - IVec2
impl Sub<IVec2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn sub(self, rhs: IVec2) -> IAabb2 {
        IAabb2 {
            min: self.min.sub(rhs),
            max: self.max.sub(rhs),
        }
    }
}

// IVec2 - IAabb2
impl Sub<IAabb2> for IVec2 {
    type Output = IAabb2;
    #[inline]
    fn sub(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.sub(rhs.min),
            max: self.sub(rhs.max),
        }
    }
}

// IAabb2 - IAabb2
impl Sub<IAabb2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn sub(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.min.sub(rhs.min),
            max: self.max.sub(rhs.max),
        }
    }
}

// IAabb2 -= IVec2
impl SubAssign<IVec2> for IAabb2 {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec2) {
        self.min.sub_assign(rhs);
        self.max.sub_assign(rhs);
    }
}

// IAabb2 -= IAabb2
impl SubAssign<IAabb2> for IAabb2 {
    #[inline]
    fn sub_assign(&mut self, rhs: IAabb2) {
        self.min.sub_assign(rhs.min);
        self.max.sub_assign(rhs.max);
    }
}

// IAabb2 * i32
impl Mul<i32> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn mul(self, rhs: i32) -> IAabb2 {
        IAabb2 {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

// i32 * IAabb2
impl Mul<IAabb2> for i32 {
    type Output = IAabb2;
    #[inline]
    fn mul(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

// IAabb2 * IVec2
impl Mul<IVec2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn mul(self, rhs: IVec2) -> IAabb2 {
        IAabb2 {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

// IVec2 * IAabb2
impl Mul<IAabb2> for IVec2 {
    type Output = IAabb2;
    #[inline]
    fn mul(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

// IAabb2 * IAabb2
impl Mul<IAabb2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn mul(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.min.mul(rhs.min),
            max: self.max.mul(rhs.max),
        }
    }
}

// IAabb2 *= i32
impl MulAssign<i32> for IAabb2 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

// IAabb2 *= IVec2
impl MulAssign<IVec2> for IAabb2 {
    #[inline]
    fn mul_assign(&mut self, rhs: IVec2) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

// IAabb2 *= IAabb2
impl MulAssign<IAabb2> for IAabb2 {
    #[inline]
    fn mul_assign(&mut self, rhs: IAabb2) {
        self.min.mul_assign(rhs.min);
        self.max.mul_assign(rhs.max);
    }
}

// IAabb2 / i32
impl Div<i32> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn div(self, rhs: i32) -> IAabb2 {
        IAabb2 {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

// i32 / IAabb2
impl Div<IAabb2> for i32 {
    type Output = IAabb2;
    #[inline]
    fn div(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

// IAabb2 / IVec2
impl Div<IVec2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn div(self, rhs: IVec2) -> IAabb2 {
        IAabb2 {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

// IVec2 / IAabb2
impl Div<IAabb2> for IVec2 {
    type Output = IAabb2;
    #[inline]
    fn div(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

// IAabb2 / IAabb2
impl Div<IAabb2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn div(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.min.div(rhs.min),
            max: self.max.div(rhs.max),
        }
    }
}

// IAabb2 /= i32
impl DivAssign<i32> for IAabb2 {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

// IAabb2 /= IVec2
impl DivAssign<IVec2> for IAabb2 {
    #[inline]
    fn div_assign(&mut self, rhs: IVec2) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

// IAabb2 /= IAabb2
impl DivAssign<IAabb2> for IAabb2 {
    #[inline]
    fn div_assign(&mut self, rhs: IAabb2) {
        self.min.div_assign(rhs.min);
        self.max.div_assign(rhs.max);
    }
}

// IAabb2 % i32
impl Rem<i32> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn rem(self, rhs: i32) -> IAabb2 {
        IAabb2 {
            min: self.min.rem(rhs),
            max: self.max.rem(rhs),
        }
    }
}

// i32 % IAabb2
impl Rem<IAabb2> for i32 {
    type Output = IAabb2;
    #[inline]
    fn rem(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.rem(rhs.min),
            max: self.rem(rhs.max),
        }
    }
}

// IAabb2 % IVec2
impl Rem<IVec2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn rem(self, rhs: IVec2) -> IAabb2 {
        IAabb2 {
            min: self.min.rem(rhs),
            max: self.max.rem(rhs),
        }
    }
}

// IVec2 % IAabb2
impl Rem<IAabb2> for IVec2 {
    type Output = IAabb2;
    #[inline]
    fn rem(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.rem(rhs.min),
            max: self.rem(rhs.max),
        }
    }
}

// IAabb2 % IAabb2
impl Rem<IAabb2> for IAabb2 {
    type Output = IAabb2;
    #[inline]
    fn rem(self, rhs: IAabb2) -> IAabb2 {
        IAabb2 {
            min: self.min.rem(rhs.min),
            max: self.max.rem(rhs.max),
        }
    }
}

// IAabb2 %= i32
impl RemAssign<i32> for IAabb2 {
    fn rem_assign(&mut self, rhs: i32) {
        self.min.rem_assign(rhs);
        self.max.rem_assign(rhs);
    }
}

// IAabb2 %= IVec2
impl RemAssign<IVec2> for IAabb2 {
    fn rem_assign(&mut self, rhs: IVec2) {
        self.min.rem_assign(rhs);
        self.max.rem_assign(rhs);
    }
}

// IAabb2 %= IAabb2
impl RemAssign<IAabb2> for IAabb2 {
    #[inline]
    fn rem_assign(&mut self, rhs: IAabb2) {
        self.min.rem_assign(rhs.min);
        self.max.rem_assign(rhs.max);
    }
}

impl AsRef<[IVec2; 2]> for IAabb2 {
    #[inline]
    fn as_ref(&self) -> &[IVec2; 2] {
        unsafe { &*(self as *const IAabb2 as *const [IVec2; 2]) }
    }
}

impl AsMut<[IVec2; 2]> for IAabb2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [IVec2; 2] {
        unsafe { &mut *(self as *mut IAabb2 as *mut [IVec2; 2]) }
    }
}

impl iter::Sum for IAabb2 {
    #[inline]
    fn sum<I: Iterator<Item = IAabb2>>(iter: I) -> IAabb2 {
        iter.fold(IAabb2::ZERO, IAabb2::add)
    }
}

impl<'a> iter::Sum<&'a IAabb2> for IAabb2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a IAabb2>>(iter: I) -> IAabb2 {
        iter.fold(IAabb2::ZERO, |a, &b| IAabb2::add(a, b))
    }
}

impl iter::Product for IAabb2 {
    #[inline]
    fn product<I: Iterator<Item = IAabb2>>(iter: I) -> IAabb2 {
        iter.fold(IAabb2::ZERO, IAabb2::mul)
    }
}

impl<'a> iter::Product<&'a IAabb2> for IAabb2 {
    #[inline]
    fn product<I: Iterator<Item = &'a IAabb2>>(iter: I) -> IAabb2 {
        iter.fold(IAabb2::ZERO, |a, &b| IAabb2::mul(a, b))
    }
}

impl Index<usize> for IAabb2 {
    type Output = IVec2;
    #[inline]
    fn index(&self, index: usize) -> &IVec2 {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!("index out of rect"),
        }
    }
}

impl IndexMut<usize> for IAabb2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut IVec2 {
        match index {
            0 => &mut self.min,
            1 => &mut self.max,
            _ => panic!("index out of rect"),
        }
    }
}

impl From<[IVec2; 2]> for IAabb2 {
    #[inline]
    fn from(value: [IVec2; 2]) -> IAabb2 {
        IAabb2 {
            min: value[0],
            max: value[1],
        }
    }
}

impl From<IAabb2> for [IVec2; 2] {
    #[inline]
    fn from(value: IAabb2) -> [IVec2; 2] {
        [value.min, value.max]
    }
}

impl From<(IVec2, IVec2)> for IAabb2 {
    #[inline]
    fn from(value: (IVec2, IVec2)) -> IAabb2 {
        IAabb2 {
            min: value.0,
            max: value.1,
        }
    }
}

impl From<IAabb2> for (IVec2, IVec2) {
    #[inline]
    fn from(value: IAabb2) -> (IVec2, IVec2) {
        (value.min, value.max)
    }
}

#[derive(Clone, Debug)]
pub struct IterPoint {
    rect: IAabb2,
    point: IVec2,
}

impl Iterator for IterPoint {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.y < self.rect.max.y {
            let n = if self.point.x + 1 < self.rect.max.x {
                ivec2(self.point.x + 1, self.point.y)
            } else {
                ivec2(self.rect.min.x, self.point.y + 1)
            };
            Some(mem::replace(&mut self.point, n))
        } else {
            None
        }
    }
}

impl ExactSizeIterator for IterPoint {
    #[inline]
    fn len(&self) -> usize {
        (self.rect.max.x - 1 - self.point.x
            + (self.rect.max.y - 1 - self.point.y) * (self.rect.max.y - self.rect.min.y)
            + 1) as usize
    }
}

impl iter::FusedIterator for IterPoint {}
