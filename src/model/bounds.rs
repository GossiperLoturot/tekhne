use glam::*;
use std::ops::*;

#[inline]
pub fn ray3a(start: Vec3A, end: Vec3A) -> Ray3A {
    Ray3A { start, end }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray3A {
    pub start: Vec3A,
    pub end: Vec3A,
}

impl Ray3A {
    #[inline]
    pub fn new(start: Vec3A, end: Vec3A) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }
}

impl Neg for Ray3A {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            start: self.start.neg(),
            end: self.end.neg(),
        }
    }
}

impl Add<Vec3A> for Ray3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Vec3A) -> Self::Output {
        Self {
            start: self.start.add(rhs),
            end: self.end.add(rhs),
        }
    }
}

impl AddAssign<Vec3A> for Ray3A {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3A) {
        self.start.add_assign(rhs);
        self.end.add_assign(rhs);
    }
}

impl Add<Ray3A> for Vec3A {
    type Output = Ray3A;
    #[inline]
    fn add(self, rhs: Ray3A) -> Self::Output {
        Ray3A {
            start: self.add(rhs.start),
            end: self.add(rhs.end),
        }
    }
}

impl Sub<f32> for Ray3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            start: self.start.sub(rhs),
            end: self.end.sub(rhs),
        }
    }
}

impl SubAssign<Vec3A> for Ray3A {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3A) {
        self.start.sub_assign(rhs);
        self.end.sub_assign(rhs);
    }
}

impl Sub<Ray3A> for Vec3A {
    type Output = Ray3A;
    #[inline]
    fn sub(self, rhs: Ray3A) -> Self::Output {
        Ray3A {
            start: self.sub(rhs.start),
            end: self.sub(rhs.end),
        }
    }
}

impl Mul<Vec3A> for Ray3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Self::Output {
        Self {
            start: self.start.mul(rhs),
            end: self.end.mul(rhs),
        }
    }
}

impl MulAssign<Vec3A> for Ray3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3A) {
        self.start.mul_assign(rhs);
        self.end.mul_assign(rhs);
    }
}

impl Mul<Ray3A> for Vec3A {
    type Output = Ray3A;
    #[inline]
    fn mul(self, rhs: Ray3A) -> Self::Output {
        Ray3A {
            start: self.mul(rhs.start),
            end: self.mul(rhs.end),
        }
    }
}

impl Mul<f32> for Ray3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            start: self.start.mul(rhs),
            end: self.end.mul(rhs),
        }
    }
}

impl MulAssign<f32> for Ray3A {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.start.mul_assign(rhs);
        self.end.mul_assign(rhs);
    }
}

impl Mul<Ray3A> for f32 {
    type Output = Ray3A;
    #[inline]
    fn mul(self, rhs: Ray3A) -> Self::Output {
        Ray3A {
            start: self.mul(rhs.start),
            end: self.mul(rhs.end),
        }
    }
}

impl Div<Vec3A> for Ray3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Vec3A) -> Self::Output {
        Self {
            start: self.start.div(rhs),
            end: self.end.div(rhs),
        }
    }
}

impl DivAssign<Vec3A> for Ray3A {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3A) {
        self.start.div_assign(rhs);
        self.end.div_assign(rhs);
    }
}

impl Div<Ray3A> for Vec3A {
    type Output = Ray3A;
    #[inline]
    fn div(self, rhs: Ray3A) -> Self::Output {
        Ray3A {
            start: self.div(rhs.start),
            end: self.div(rhs.end),
        }
    }
}

impl Div<f32> for Ray3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            start: self.start.div(rhs),
            end: self.end.div(rhs),
        }
    }
}

impl DivAssign<f32> for Ray3A {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.start.div_assign(rhs);
        self.end.div_assign(rhs);
    }
}

impl Div<Ray3A> for f32 {
    type Output = Ray3A;
    #[inline]
    fn div(self, rhs: Ray3A) -> Self::Output {
        Ray3A {
            start: self.div(rhs.start),
            end: self.div(rhs.end),
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

pub trait Intersect<T> {
    fn intersect(self, other: T) -> bool;
}

impl Intersect<Vec3A> for Aabb3A {
    #[inline]
    fn intersect(self, other: Vec3A) -> bool {
        self.min.x <= other.x
            && other.x <= self.max.x
            && self.min.y <= other.y
            && other.y <= self.max.y
            && self.min.z <= other.z
            && other.z <= self.max.z
    }
}

impl Intersect<Ray3A> for Aabb3A {
    #[inline]
    fn intersect(self, other: Ray3A) -> bool {
        let near = (self.min - other.start) / (other.end - other.start);
        let far = (self.max - other.start) / (other.end - other.start);
        let (near, far) = (Vec3A::min(near, far), Vec3A::max(near, far));
        near.max_element() - far.min_element() <= 0.0
    }
}

impl Intersect<Aabb3A> for Aabb3A {
    #[inline]
    fn intersect(self, other: Aabb3A) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.min.z <= other.max.z
            && other.min.x <= self.max.x
            && other.min.y <= self.max.y
            && other.min.z <= self.max.z
    }
}

impl Intersect<IVec3> for IAabb3 {
    #[inline]
    fn intersect(self, other: IVec3) -> bool {
        self.min.x <= other.x
            && other.x <= self.max.x
            && self.min.y <= other.y
            && other.y <= self.max.y
            && self.min.z <= other.z
            && other.z <= self.max.z
    }
}

impl Intersect<IAabb3> for IAabb3 {
    #[inline]
    fn intersect(self, other: IAabb3) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.min.z <= other.max.z
            && other.min.x <= self.max.x
            && other.min.y <= self.max.y
            && other.min.z <= self.max.z
    }
}
