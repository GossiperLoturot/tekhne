//! エンティティシステムの機能に関するモジュール

use aabb::*;
use ahash::{HashMap, HashSet};
use glam::*;
use slab::Slab;

/// エンティティの種類
#[derive(Clone, Copy, Debug)]
pub enum EntityKind {
    Player,
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceStone,
    SurfaceSand,
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

impl EntityKind {
    /// エンティティの種類における破壊可能性を返す。
    #[inline]
    pub fn breakable(&self) -> bool {
        match self {
            Self::Player => false,
            Self::SurfaceDirt => false,
            Self::SurfaceGrass => false,
            Self::SurfaceGravel => false,
            Self::SurfaceStone => false,
            Self::SurfaceSand => false,
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

    /// エンティティの種類における大きさを返す。
    #[inline]
    pub fn bounds(&self) -> Vec2 {
        match self {
            Self::Player => vec2(1.0, 2.0),
            Self::SurfaceDirt => vec2(1.0, 1.0),
            Self::SurfaceGrass => vec2(1.0, 1.0),
            Self::SurfaceGravel => vec2(1.0, 1.0),
            Self::SurfaceStone => vec2(1.0, 1.0),
            Self::SurfaceSand => vec2(1.0, 1.0),
            Self::MixGrass => vec2(1.0, 1.0),
            Self::Dandelion => vec2(1.0, 1.0),
            Self::FallenBranch => vec2(1.0, 1.0),
            Self::FallenLeaves => vec2(1.0, 1.0),
            Self::MixPebbles => vec2(1.0, 1.0),
            Self::OakTree => vec2(3.0, 4.0),
            Self::BirchTree => vec2(3.0, 4.0),
            Self::DyingTree => vec2(3.0, 4.0),
            Self::FallenTree => vec2(4.0, 2.0),
            Self::MixRock => vec2(2.0, 2.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub kind: EntityKind,
    pub position: Vec2,
}

impl Entity {
    /// 新しいエンティティを作成する。
    #[inline]
    pub fn new(position: Vec2, kind: EntityKind) -> Self {
        Self { position, kind }
    }

    /// エンティティの破壊可能性を返す。
    #[inline]
    pub fn breakable(&self) -> bool {
        self.kind.breakable()
    }

    /// エンティティの大きさを返す。
    #[inline]
    pub fn bounds(&self) -> Aabb2 {
        aabb2(self.position, self.position + self.kind.bounds())
    }
}

/// エンティティシステムの機能
#[derive(Default)]
pub struct EntitySystem {
    entities: Slab<Entity>,
    index: HashMap<IVec2, HashSet<usize>>,
}

impl EntitySystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: f32 = 32.0;

    /// エンティティを追加し、識別子を返す。
    pub fn insert(&mut self, entity: Entity) -> usize {
        let id = self.entities.vacant_key();

        // インデクスを構築
        grid_point_iterator(entity.bounds(), Self::GRID_SIZE).for_each(|point| {
            self.index.entry(point).or_default().insert(id);
        });

        self.entities.insert(entity)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, id: usize) -> Option<Entity> {
        let entity = self.entities.try_remove(id)?;

        // インデクスを破棄
        grid_point_iterator(entity.bounds(), Self::GRID_SIZE).for_each(|point| {
            self.index.get_mut(&point).unwrap().remove(&id);
        });

        Some(entity)
    }

    /// 指定した識別子に対応するエンティティの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// 指定した識別子に対応するエンティティの参照を返す。
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    pub fn get_from_area(&self, bounds: Aabb2) -> impl Iterator<Item = (usize, &Entity)> {
        grid_point_iterator(bounds, Self::GRID_SIZE)
            .filter_map(|point| self.index.get(&point))
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|&id| {
                let entity = self.entities.get(id).unwrap();
                (id, entity)
            })
    }
}

/// 指定した範囲をすべて含むグリッドに存在する整数点のイテレータ
fn grid_point_iterator(bounds: Aabb2, grid_size: f32) -> impl Iterator<Item = IVec2> {
    let bounds = bounds.div_euclid_f32(grid_size);
    let min = bounds.min.as_ivec2();
    let max = bounds.max.as_ivec2();
    (min.x..=max.x).flat_map(move |x| (min.y..=max.y).map(move |y| ivec2(x, y)))
}
