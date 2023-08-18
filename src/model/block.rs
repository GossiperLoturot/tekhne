//! ブロック単体に関するモジュール

use super::{iaabb3, IAabb3};
use glam::*;

/// ブロックの種類
#[derive(Debug, Clone, Copy)]
pub enum BlockKind {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
    MixGrass,
    Dandelion,
    FallenBranch,
    FallenLeaves,
    MixPebbles,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl BlockKind {
    /// ブロックの破壊可能性を返す。
    ///
    /// 破壊可能ならばtrueを返し、そうでない場合はfalseを返す。
    #[inline]
    pub fn breakable(&self) -> bool {
        match self {
            Self::SurfaceDirt => false,
            Self::SurfaceGrass => false,
            Self::SurfaceGravel => false,
            Self::SurfaceSand => false,
            Self::SurfaceStone => false,
            Self::MixGrass => true,
            Self::Dandelion => true,
            Self::FallenBranch => true,
            Self::FallenLeaves => true,
            Self::MixPebbles => true,
            Self::OakTree => true,
            Self::BirchTree => true,
            Self::DyingTree => true,
            Self::FallenTree => true,
            Self::MixRock => true,
        }
    }

    /// ブロックの衝突判定領域を返す。
    ///
    /// ローカル座標空間(ブロックが設置されるであろう位置を原点とした座標空間)上でのAABBを返す。
    #[inline]
    pub fn bounds(&self) -> IAabb3 {
        match self {
            Self::SurfaceDirt => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceGrass => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceGravel => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceSand => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceStone => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::MixGrass => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::Dandelion => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::FallenBranch => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::FallenLeaves => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::MixPebbles => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::OakTree => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 2)),
            Self::BirchTree => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 2)),
            Self::DyingTree => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 2)),
            Self::FallenTree => iaabb3(ivec3(0, 0, 0), ivec3(3, 1, 0)),
            Self::MixRock => iaabb3(ivec3(0, 0, 0), ivec3(1, 1, 0)),
        }
    }
}

/// ワールドに配置されるブロックのデータ
#[derive(Debug, Clone)]
pub struct Block {
    pub position: IVec3,
    pub kind: BlockKind,
}

impl Block {
    /// 新しいブロックを作成する。
    #[inline]
    pub fn new(position: IVec3, kind: BlockKind) -> Self {
        Self { position, kind }
    }

    /// ブロックの破壊可能性を返す。
    ///
    /// 破壊可能ならばtrueを返し、そうでない場合はfalseを返す。
    #[inline]
    pub fn breakable(&self) -> bool {
        self.kind.breakable()
    }

    /// ブロックの衝突判定領域を返す。
    ///
    /// ワールド座標空間上でのAABBを返す。
    #[inline]
    pub fn bounds(&self) -> IAabb3 {
        self.position + self.kind.bounds()
    }
}
