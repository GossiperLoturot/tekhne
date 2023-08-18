//! カメラ単体に関するモジュール

use super::{aabb3a, Aabb3A};
use glam::*;

/// ワールドに配置されるカメラのデータ
///
/// 原点を中心としてその周りを俯瞰するような描写領域を形成する。
#[derive(Debug, Clone)]
pub struct Camera {
    /// 原点
    pub position: Vec3A,
    /// 原点から離れる距離
    pub zoom: f32,
}

impl Camera {
    /// 新しいカメラを作成する。
    #[inline]
    pub fn new(position: Vec3A, zoom: f32) -> Self {
        Self { position, zoom }
    }

    /// 描写領域を完全に覆うようなビュー行列を計算する。
    ///
    /// 3Dから2Dへの射影行列には透視・平行投影では表せない特別な行列を使用する。
    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        let view_aabb = self.view_bounds();
        Mat4::orthographic_rh(
            view_aabb.min.x,
            view_aabb.max.x,
            view_aabb.min.y,
            view_aabb.max.y,
            view_aabb.min.z,
            view_aabb.max.z,
        ) * mat4(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 1.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// 描写領域を計算する。
    #[inline]
    pub fn view_bounds(&self) -> Aabb3A {
        aabb3a(
            self.position - Vec3A::splat(self.zoom),
            self.position + Vec3A::splat(self.zoom),
        )
    }
}
