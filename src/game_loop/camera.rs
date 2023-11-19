use aabb::*;
use glam::*;

pub enum CameraSystem {
    NotPresent,
    Present { position: Vec2, zoom: f32 },
}

impl CameraSystem {
    /// Z値クリップの最小値
    const Z_NEAR: f32 = -32.0;

    /// Z値クリップの最大値
    const Z_FAR: f32 = 32.0;

    /// 新しいカメラシステムを作成する。
    #[inline]
    pub fn new() -> Self {
        Self::NotPresent
    }

    /// 描写範囲を返す。
    #[inline]
    pub fn view_bounds(&self) -> Option<Aabb2> {
        match self {
            Self::Present { position, zoom, .. } => {
                Some(aabb2(*position - *zoom, *position + *zoom))
            }
            _ => None,
        }
    }

    /// 描写範囲のビュー行列を返す。
    #[inline]
    pub fn view_matrix(&self) -> Option<Mat4> {
        match self.view_bounds() {
            Some(bounds) => Some(Mat4::orthographic_rh(
                bounds.min.x,
                bounds.max.x,
                bounds.min.y,
                bounds.max.y,
                Self::Z_NEAR,
                Self::Z_FAR,
            )),
            None => None,
        }
    }
}
