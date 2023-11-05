//! エンティティシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::assets;

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
    grid_index: HashMap<IVec2, Slab<usize>>,
    grid_index_rev: Slab<usize>,
}

impl EntitySystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            grid_index: Default::default(),
            grid_index_rev: Default::default(),
        }
    }

    /// 指定した範囲にエンティティが存在するか真偽値を返す。
    #[inline]
    pub fn contains(&self, assets: &assets::Assets, bounds: Aabb2) -> bool {
        self.get_from_bounds(assets, bounds).next().is_some()
    }

    /// エンティティを追加し、識別子を返す。
    pub fn insert(&mut self, assets: &assets::Assets, entity: Entity) -> Option<usize> {
        let spec = &assets.entity_specs[entity.spec_id];
        let bounds = aabb2(entity.position, entity.position + spec.size);

        // 重複の回避
        if self.contains(assets, bounds) {
            return None;
        }

        let id = self.entities.vacant_key();

        // インデクスを構築
        let grid_point = entity
            .position
            .div_euclid(Vec2::splat(Self::GRID_SIZE))
            .as_ivec2();
        let idx = self.grid_index.entry(grid_point).or_default().insert(id);
        self.grid_index_rev.insert(idx);

        self.entities.insert(entity);
        Some(id)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, assets: &assets::Assets, id: usize) -> Option<Entity> {
        let entity = self.entities.try_remove(id)?;

        // インデクスを破棄
        let idx = self.grid_index_rev.remove(id);
        let grid_point = entity
            .position
            .div_euclid(Vec2::splat(Self::GRID_SIZE))
            .as_ivec2();
        self.grid_index.get_mut(&grid_point).unwrap().remove(idx);

        Some(entity)
    }

    /// 指定した識別子に対応するエンティティの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    #[inline]
    pub fn get_from_bounds<'a>(
        &'a self,
        assets: &'a assets::Assets,
        bounds: Aabb2,
    ) -> impl Iterator<Item = (usize, &'a Entity)> {
        let grid_bounds = bounds.div_euclid_f32(Self::GRID_SIZE).as_iaabb2();
        let (min, max) = (grid_bounds.min - IVec2::ONE, grid_bounds.max + IVec2::ONE);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |grid_point| self.grid_index.get(&grid_point))
            .flatten()
            .map(|(_, &id)| (id, &self.entities[id]))
            .filter(move |(_, entity)| {
                let spec = &assets.entity_specs[entity.spec_id];
                let self_bounds = aabb2(entity.position, entity.position + spec.size);
                bounds.intersect(self_bounds)
            })
    }
}
