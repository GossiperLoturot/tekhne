//! エンティティシステムの機能に関するモジュール

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
    pub fn size(&self) -> Vec2 {
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
            Self::OakTree => vec2(1.0, 1.0),
            Self::BirchTree => vec2(4.0, 4.0),
            Self::DyingTree => vec2(4.0, 4.0),
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
    pub fn size(&self) -> Vec2 {
        self.kind.size()
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
    const GRID_SIZE: Vec2 = vec2(32.0, 32.0);

    /// エンティティを追加し、識別子を返す。
    pub fn insert(&mut self, entity: Entity) -> usize {
        let id = self.entities.vacant_key();

        // インデクスを構築
        let start = entity.position.div_euclid(Self::GRID_SIZE).as_ivec2();
        let end = (entity.position + entity.size())
            .div_euclid(Self::GRID_SIZE)
            .as_ivec2();
        cross_iterator(start.x..=end.x, start.y..=end.y).for_each(|(x, y)| {
            self.index.entry(ivec2(x, y)).or_default().insert(id);
        });

        self.entities.insert(entity)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, id: usize) -> Option<Entity> {
        let entity = self.entities.try_remove(id)?;

        // インデクスを破棄
        let start = entity.position.div_euclid(Self::GRID_SIZE).as_ivec2();
        let end = (entity.position + entity.size())
            .div_euclid(Self::GRID_SIZE)
            .as_ivec2();
        cross_iterator(start.x..=end.x, start.y..=end.y).for_each(|(x, y)| {
            self.index.get_mut(&ivec2(x, y)).unwrap().remove(&id);
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
    pub fn get_from_area(&self, start: Vec2, end: Vec2) -> impl Iterator<Item = (usize, &Entity)> {
        let istart = start.div_euclid(Self::GRID_SIZE).as_ivec2();
        let iend = end.div_euclid(Self::GRID_SIZE).as_ivec2();
        cross_iterator(istart.x..=iend.x, istart.y..=iend.y)
            .filter_map(|(x, y)| self.index.get(&ivec2(x, y)))
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|&id| {
                let entity = self.entities.get(id).unwrap();
                (id, entity)
            })
            .filter(move |(_, entity)| {
                let entity_start = entity.position;
                let entity_end = entity.position + entity.size();
                start.x <= entity_end.x
                    && entity_start.x <= end.x
                    && start.y <= entity_end.y
                    && entity_start.y <= end.y
            })
    }
}

/// 2つのイテレータの直積を返す。
fn cross_iterator<T, U, V, W>(first: T, second: U) -> impl Iterator<Item = (V, W)>
where
    T: Iterator<Item = V> + Clone,
    U: Iterator<Item = W> + Clone,
    V: Copy,
    W: Copy,
{
    first
        .map(move |v| second.clone().map(move |w| (v, w)))
        .flatten()
}
