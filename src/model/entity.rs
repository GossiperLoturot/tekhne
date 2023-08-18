//! エンティティ単体に関するモジュール

use super::{aabb3a, Aabb3A};
use glam::*;

/// エンティティの種類
#[derive(Debug, Clone, Copy)]
pub enum EntityKind {
    Player,
}

impl EntityKind {
    /// エンティティの破壊可能性を返す。
    ///
    /// 破壊可能ならばtrueを返し、そうでない場合はfalseを返す。
    #[inline]
    pub fn breakable(&self) -> bool {
        match self {
            EntityKind::Player => false,
        }
    }

    /// エンティティの当たり判定を返す。
    ///
    /// ローカル座標空間(ブロックが設置されるであろう位置を原点とした座標空間)上でのAABBを返す。
    #[inline]
    pub fn bounds(&self) -> Aabb3A {
        match self {
            EntityKind::Player => aabb3a(vec3a(-0.5, -0.5, 0.0), vec3a(0.5, 0.5, 2.0)),
        }
    }
}

/// ワールドに配置されるエンティティのデータ
#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Vec3A,
    pub kind: EntityKind,
}

impl Entity {
    /// 新しいエンティティを作成する。
    #[inline]
    pub fn new(position: Vec3A, kind: EntityKind) -> Self {
        Self { position, kind }
    }

    /// エンティティの破壊可能性を返す。
    ///
    /// 破壊可能ならばtrueを返し、そうでない場合はfalseを返す。
    #[inline]
    pub fn breakable(&self) -> bool {
        self.kind.breakable()
    }

    /// エンティティの衝突判定領域を返す。
    ///
    /// ワールド座標空間上でのAABBを返す。
    #[inline]
    pub fn aabb(&self) -> Aabb3A {
        self.position + self.kind.bounds()
    }
}
