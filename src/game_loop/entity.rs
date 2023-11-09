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

struct EntityMeta {
    entity: Entity,
    grid_index_rev: Vec<(IVec2, usize)>,
}

/// エンティティシステムの機能
pub struct EntitySystem {
    entity_metas: Slab<EntityMeta>,
    grid_index: HashMap<IVec2, Slab<usize>>,
}

impl EntitySystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            entity_metas: Default::default(),
            grid_index: Default::default(),
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

        let entity_id = self.entity_metas.vacant_key();

        // インデクスを構築
        let grid_bounds = bounds.to_grid(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min, grid_bounds.max);
        let grid_index_rev = itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .map(|grid| {
                let id = self.grid_index.entry(grid).or_default().insert(entity_id);
                (grid, id)
            })
            .collect::<Vec<_>>();

        self.entity_metas.insert(EntityMeta {
            entity,
            grid_index_rev,
        });
        Some(entity_id)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, assets: &assets::Assets, id: usize) -> Option<Entity> {
        let EntityMeta {
            entity,
            grid_index_rev,
        } = self.entity_metas.try_remove(id)?;

        // インデクスを破棄
        grid_index_rev.into_iter().for_each(|(grid, id)| {
            self.grid_index.get_mut(&grid).unwrap().remove(id);
        });

        Some(entity)
    }

    /// 指定した識別子に対応するエンティティの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entity_metas
            .get(id)
            .map(|entity_meta| &entity_meta.entity)
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    #[inline]
    pub fn get_from_bounds<'a>(
        &'a self,
        assets: &'a assets::Assets,
        bounds: Aabb2,
    ) -> impl Iterator<Item = (usize, &'a Entity)> {
        let grid_bounds = bounds.to_grid(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min, grid_bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |grid| self.grid_index.get(&grid))
            .flatten()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|(_, &id)| (id, &self.entity_metas[id].entity))
            .filter(move |(_, entity)| {
                let spec = &assets.entity_specs[entity.spec_id];
                let entity_bounds = aabb2(entity.position, entity.position + spec.size);
                bounds.intersects(entity_bounds)
            })
    }
}
