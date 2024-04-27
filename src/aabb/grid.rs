use float_next_after::NextAfter;
use glam::*;

use super::*;

/// Trait for transforming to grid space from base space.
pub trait ToGridSpace<V, E> {
    /// Returns an element on grid space from an element on base space.
    fn to_grid_space(self, size: E) -> V;
}

/// Trait for transforming to base space from grid space.
pub trait ToBaseSpace<V, E> {
    /// Returns an element on base space from an element on grid space.
    fn to_base_space(self, size: E) -> V;
}

impl ToGridSpace<IVec2, i32> for IVec2 {
    #[inline]
    fn to_grid_space(self, size: i32) -> IVec2 {
        self.div_euclid(IVec2::splat(size))
    }
}

impl ToGridSpace<IVec2, f32> for Vec2 {
    #[inline]
    fn to_grid_space(self, size: f32) -> IVec2 {
        self.div_euclid(Vec2::splat(size)).as_ivec2()
    }
}

impl ToGridSpace<IAabb2, i32> for IAabb2 {
    #[inline]
    fn to_grid_space(self, size: i32) -> IAabb2 {
        let rect = iaabb2(self.min, self.max - IVec2::ONE).div_euclid_i32(size);
        iaabb2(rect.min, rect.max + IVec2::ONE)
    }
}

impl ToGridSpace<IAabb2, f32> for Aabb2 {
    #[inline]
    fn to_grid_space(self, size: f32) -> IAabb2 {
        let max_x = self.max.x.next_after(f32::NEG_INFINITY);
        let max_y = self.max.y.next_after(f32::NEG_INFINITY);
        let rect = aabb2(self.min, vec2(max_x, max_y))
            .div_euclid_f32(size)
            .as_iaabb2();
        iaabb2(rect.min, rect.max + IVec2::ONE)
    }
}

impl ToBaseSpace<IAabb2, i32> for IVec2 {
    #[inline]
    fn to_base_space(self, size: i32) -> IAabb2 {
        iaabb2(self * size, (self + IVec2::ONE) * size)
    }
}

impl ToBaseSpace<Aabb2, f32> for IVec2 {
    #[inline]
    fn to_base_space(self, size: f32) -> Aabb2 {
        aabb2(self.as_vec2() * size, (self.as_vec2() + Vec2::ONE) * size)
    }
}

impl ToBaseSpace<IAabb2, i32> for IAabb2 {
    #[inline]
    fn to_base_space(self, size: i32) -> IAabb2 {
        iaabb2(self.min * size, self.max * size)
    }
}

impl ToBaseSpace<Aabb2, f32> for IAabb2 {
    #[inline]
    fn to_base_space(self, size: f32) -> Aabb2 {
        aabb2(self.min.as_vec2() * size, self.max.as_vec2() * size)
    }
}
