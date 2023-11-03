//! エンティティシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

pub struct Entity {
    pub spec_id: usize,
    pub position: Vec2,
}

impl Entity {
    /// 新しいエンティティを作成する。
    #[inline]
    pub fn new(spec_id: usize, position: Vec2) -> Self {
        Self { spec_id, position }
    }
}

/// エンティティシステムの機能
pub struct EntitySystem {
    entities: Slab<Entity>,
    index: HashMap<IVec2, Slab<usize>>,
    rev_index: Slab<usize>,
}

impl EntitySystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            index: Default::default(),
            rev_index: Default::default(),
        }
    }

    /// エンティティを追加し、識別子を返す。
    pub fn insert(&mut self, entity: Entity) -> usize {
        let id = self.entities.vacant_key();

        // インデクスを構築
        let point = entity
            .position
            .div_euclid(vec2(Self::GRID_SIZE, Self::GRID_SIZE))
            .as_ivec2();
        let idx_id = self.index.entry(point).or_default().insert(id);
        self.rev_index.insert(idx_id);

        self.entities.insert(entity)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, id: usize) -> Option<Entity> {
        let entity = self.entities.try_remove(id)?;

        // インデクスを破棄
        let idx_id = self.rev_index.remove(id);
        let point = entity
            .position
            .div_euclid(vec2(Self::GRID_SIZE, Self::GRID_SIZE))
            .as_ivec2();
        self.index.get_mut(&point).unwrap().remove(idx_id);

        Some(entity)
    }

    /// 指定した識別子に対応するエンティティの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    #[inline]
    pub fn get_from_area(&self, bounds: Aabb2) -> impl Iterator<Item = (usize, &Entity)> {
        let grid_bounds = bounds.div_euclid_f32(Self::GRID_SIZE);
        let min = grid_bounds.min.as_ivec2();
        let max = grid_bounds.max.as_ivec2();
        let iter = (min.x..=max.x).flat_map(move |x| (min.y..=max.y).map(move |y| ivec2(x, y)));

        iter.filter_map(move |point| self.index.get(&point))
            .flatten()
            .map(|(_, &id)| (id, &self.entities[id]))
    }
}
