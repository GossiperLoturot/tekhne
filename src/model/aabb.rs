use glam::*;
use std::ops::*;

#[inline]
pub fn iaabb3(min: IVec3, max: IVec3) -> IAabb3 {
    IAabb3 { min, max }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IAabb3 {
    pub min: IVec3,
    pub max: IVec3,
}

impl IAabb3 {
    #[inline]
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn volume(&self) -> IVec3 {
        self.min - self.max
    }

    #[inline]
    pub fn contains(&self, point: IVec3) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.y <= point.y
            && point.y <= self.max.y
            && self.min.z <= point.z
            && point.z <= self.max.z
    }

    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.min.z <= other.max.z
            && other.min.x <= self.max.x
            && other.min.y <= self.max.y
            && other.min.z <= self.max.z
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = IVec3> + '_ {
        (self.min.x..=self.max.x)
            .flat_map(|x| (self.min.y..=self.max.y).map(move |y| (x, y)))
            .flat_map(|(x, y)| (self.min.z..=self.max.z).map(move |z| (x, y, z)))
            .map(|(x, y, z)| ivec3(x, y, z))
    }

    #[inline]
    pub fn as_aabb3a(&self) -> Aabb3A {
        Aabb3A {
            min: self.min.as_vec3a(),
            max: self.max.as_vec3a(),
        }
    }
}

impl Neg for IAabb3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            min: self.min.neg(),
            max: self.max.neg(),
        }
    }
}

impl Add<IVec3> for IAabb3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: IVec3) -> Self::Output {
        Self {
            min: self.min.add(rhs),
            max: self.max.add(rhs),
        }
    }
}

impl AddAssign<IVec3> for IAabb3 {
    #[inline]
    fn add_assign(&mut self, rhs: IVec3) {
        self.min.add_assign(rhs);
        self.max.add_assign(rhs);
    }
}

impl Add<IAabb3> for IVec3 {
    type Output = IAabb3;
    #[inline]
    fn add(self, rhs: IAabb3) -> Self::Output {
        IAabb3 {
            min: self.add(rhs.min),
            max: self.add(rhs.max),
        }
    }
}

impl Sub<i32> for IAabb3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min.sub(rhs),
            max: self.max.sub(rhs),
        }
    }
}

impl SubAssign<IVec3> for IAabb3 {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec3) {
        self.min.sub_assign(rhs);
        self.max.sub_assign(rhs);
    }
}

impl Sub<IAabb3> for IVec3 {
    type Output = IAabb3;
    #[inline]
    fn sub(self, rhs: IAabb3) -> Self::Output {
        IAabb3 {
            min: self.sub(rhs.min),
            max: self.sub(rhs.max),
        }
    }
}

impl Mul<IVec3> for IAabb3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: IVec3) -> Self::Output {
        Self {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

impl MulAssign<IVec3> for IAabb3 {
    #[inline]
    fn mul_assign(&mut self, rhs: IVec3) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

impl Mul<IAabb3> for IVec3 {
    type Output = IAabb3;
    #[inline]
    fn mul(self, rhs: IAabb3) -> Self::Output {
        IAabb3 {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

impl Mul<i32> for IAabb3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

impl MulAssign<i32> for IAabb3 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

impl Mul<IAabb3> for i32 {
    type Output = IAabb3;
    #[inline]
    fn mul(self, rhs: IAabb3) -> Self::Output {
        IAabb3 {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

impl Div<IVec3> for IAabb3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: IVec3) -> Self::Output {
        Self {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

impl DivAssign<IVec3> for IAabb3 {
    #[inline]
    fn div_assign(&mut self, rhs: IVec3) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

impl Div<IAabb3> for IVec3 {
    type Output = IAabb3;
    #[inline]
    fn div(self, rhs: IAabb3) -> Self::Output {
        IAabb3 {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

impl Div<i32> for IAabb3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

impl DivAssign<i32> for IAabb3 {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

impl Div<IAabb3> for i32 {
    type Output = IAabb3;
    #[inline]
    fn div(self, rhs: IAabb3) -> Self::Output {
        IAabb3 {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

#[inline]
pub fn aabb3a(min: Vec3A, max: Vec3A) -> Aabb3A {
    Aabb3A { min, max }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Aabb3A {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl Aabb3A {
    #[inline]
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn volume(&self) -> Vec3A {
        self.min - self.max
    }

    #[inline]
    pub fn contains(&self, point: Vec3A) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.y <= point.y
            && point.y <= self.max.y
            && self.min.z <= point.z
            && point.z <= self.max.z
    }

    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.min.z <= other.max.z
            && other.min.x <= self.max.x
            && other.min.y <= self.max.y
            && other.min.z <= self.max.z
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Self {
            min: self.min.floor(),
            max: self.max.floor(),
        }
    }

    #[inline]
    pub fn round(&self) -> Self {
        Self {
            min: self.min.round(),
            max: self.max.round(),
        }
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Self {
            min: self.min.ceil(),
            max: self.max.ceil(),
        }
    }

    #[inline]
    pub fn as_iaabb3(&self) -> IAabb3 {
        IAabb3 {
            min: self.min.as_ivec3(),
            max: self.max.as_ivec3(),
        }
    }
}

impl Neg for Aabb3A {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            min: self.min.neg(),
            max: self.max.neg(),
        }
    }
}

impl Add<Vec3A> for Aabb3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Vec3A) -> Self::Output {
        Self {
            min: self.min.add(rhs),
            max: self.max.add(rhs),
        }
    }
}

impl AddAssign<Vec3A> for Aabb3A {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3A) {
        self.min.add_assign(rhs);
        self.max.add_assign(rhs);
    }
}

impl Add<Aabb3A> for Vec3A {
    type Output = Aabb3A;
    #[inline]
    fn add(self, rhs: Aabb3A) -> Self::Output {
        Aabb3A {
            min: self.add(rhs.min),
            max: self.add(rhs.max),
        }
    }
}

impl Sub<f32> for Aabb3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min.sub(rhs),
            max: self.max.sub(rhs),
        }
    }
}

impl SubAssign<Vec3A> for Aabb3A {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3A) {
        self.min.sub_assign(rhs);
        self.max.sub_assign(rhs);
    }
}

impl Sub<Aabb3A> for Vec3A {
    type Output = Aabb3A;
    #[inline]
    fn sub(self, rhs: Aabb3A) -> Self::Output {
        Aabb3A {
            min: self.sub(rhs.min),
            max: self.sub(rhs.max),
        }
    }
}

impl Mul<Vec3A> for Aabb3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Self::Output {
        Self {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

impl MulAssign<Vec3A> for Aabb3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3A) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

impl Mul<Aabb3A> for Vec3A {
    type Output = Aabb3A;
    #[inline]
    fn mul(self, rhs: Aabb3A) -> Self::Output {
        Aabb3A {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

impl Mul<f32> for Aabb3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min.mul(rhs),
            max: self.max.mul(rhs),
        }
    }
}

impl MulAssign<f32> for Aabb3A {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.min.mul_assign(rhs);
        self.max.mul_assign(rhs);
    }
}

impl Mul<Aabb3A> for f32 {
    type Output = Aabb3A;
    #[inline]
    fn mul(self, rhs: Aabb3A) -> Self::Output {
        Aabb3A {
            min: self.mul(rhs.min),
            max: self.mul(rhs.max),
        }
    }
}

impl Div<Vec3A> for Aabb3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Vec3A) -> Self::Output {
        Self {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

impl DivAssign<Vec3A> for Aabb3A {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3A) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

impl Div<Aabb3A> for Vec3A {
    type Output = Aabb3A;
    #[inline]
    fn div(self, rhs: Aabb3A) -> Self::Output {
        Aabb3A {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

impl Div<f32> for Aabb3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            min: self.min.div(rhs),
            max: self.max.div(rhs),
        }
    }
}

impl DivAssign<f32> for Aabb3A {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.min.div_assign(rhs);
        self.max.div_assign(rhs);
    }
}

impl Div<Aabb3A> for f32 {
    type Output = Aabb3A;
    #[inline]
    fn div(self, rhs: Aabb3A) -> Self::Output {
        Aabb3A {
            min: self.div(rhs.min),
            max: self.div(rhs.max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_inside() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let point = ivec3(1, 2, 3);
        assert!(aabb.contains(point));
    }

    #[test]
    fn contains_on_border_a() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let point = ivec3(0, 0, 0);
        assert!(aabb.contains(point));
    }

    #[test]
    fn contains_on_border_b() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let point = ivec3(8, 8, 8);
        assert!(aabb.contains(point));
    }

    #[test]
    fn contains_outside() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let point = ivec3(9, 10, 11);
        assert!(!aabb.contains(point));
    }

    #[test]
    fn intersects_inside() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(2, 2, 2), ivec3(6, 6, 6));
        assert!(aabb.intersects(other_aabb));
    }

    #[test]
    fn intersects_a() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(-4, -4, -4), ivec3(4, 4, 4));
        assert!(aabb.intersects(other_aabb));
    }

    #[test]
    fn intersects_b() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(-8, 2, 2), ivec3(2, 6, 6));
        assert!(aabb.intersects(other_aabb));
    }

    #[test]
    fn intersects_on_border_a() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(-8, 0, 0), ivec3(0, 8, 8));
        assert!(aabb.intersects(other_aabb));
    }

    #[test]
    fn intersects_on_border_b() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(-8, -8, -8), ivec3(0, 0, 0));
        assert!(aabb.intersects(other_aabb));
    }

    #[test]
    fn intersects_outside_a() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(-8, -8, -8), ivec3(-4, -4, -4));
        assert!(!aabb.intersects(other_aabb));
    }

    #[test]
    fn intersects_outside_b() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        let other_aabb = iaabb3(ivec3(-8, 2, 2), ivec3(-4, 6, 6));
        assert!(!aabb.intersects(other_aabb));
    }

    #[test]
    fn iterator() {
        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        assert_eq!(aabb.iter().count(), 729);
    }
}
