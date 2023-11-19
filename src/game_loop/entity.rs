//! エンティティシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::assets;

pub enum Bounds {
    Logic(Aabb2),
    View(Aabb2),
}

#[derive(Clone)]
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
    logic_grid_index_rev: Vec<(IVec2, usize)>,
    view_grid_index_rev: Vec<(IVec2, usize)>,
}

/// エンティティシステムの機能
pub struct EntitySystem {
    entity_metas: Slab<EntityMeta>,
    logic_grid_index: HashMap<IVec2, Slab<usize>>,
    view_grid_index: HashMap<IVec2, Slab<usize>>,
}

impl EntitySystem {
    /// 近傍探索のための空間分割サイズ
    const LOGIC_GRID_SIZE: f32 = 32.0;

    /// 近傍探索のための空間分割サイズ
    const VIEW_GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            entity_metas: Default::default(),
            logic_grid_index: Default::default(),
            view_grid_index: Default::default(),
        }
    }

    /// エンティティを追加し、識別子を返す。
    pub fn insert(&mut self, assets: &assets::Assets, entity: Entity) -> Option<usize> {
        let spec = &assets.entity_specs[entity.spec_id];

        // 重複の回避
        let bounds = aabb2(entity.position, entity.position + spec.logic_size);
        if self.exists_by_bounds(assets, Bounds::Logic(bounds)) {
            return None;
        }

        let entity_id = self.entity_metas.vacant_key();

        // インデクスを構築 (1)
        let bounds = aabb2(entity.position, entity.position + spec.logic_size);
        let logic_grid_index_rev = bounds
            .to_grid_space(Self::LOGIC_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .logic_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(entity_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        // インデクスを構築 (2)
        let bounds = entity.position + spec.view_size;
        let view_grid_index_rev = bounds
            .to_grid_space(Self::VIEW_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .view_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(entity_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        self.entity_metas.insert(EntityMeta {
            entity,
            logic_grid_index_rev,
            view_grid_index_rev,
        });
        Some(entity_id)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, assets: &assets::Assets, id: usize) -> Option<Entity> {
        let EntityMeta {
            entity,
            logic_grid_index_rev,
            view_grid_index_rev,
        } = self.entity_metas.try_remove(id)?;

        // インデクスを破棄 (1)
        logic_grid_index_rev
            .into_iter()
            .for_each(|(grid_point, id)| {
                self.logic_grid_index
                    .get_mut(&grid_point)
                    .unwrap()
                    .remove(id);
            });

        // インデクスを破棄 (2)
        view_grid_index_rev
            .into_iter()
            .for_each(|(grid_point, id)| {
                self.view_grid_index
                    .get_mut(&grid_point)
                    .unwrap()
                    .remove(id);
            });

        Some(entity)
    }

    /// 指定した識別子に対応するエンティティの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entity_metas
            .get(id)
            .map(|entity_meta| &entity_meta.entity)
    }

    /// 指定した範囲にエンティティが存在するか真偽値を返す。
    #[inline]
    pub fn exists_by_bounds(&self, assets: &assets::Assets, bounds: Bounds) -> bool {
        self.get_by_bounds(assets, bounds).next().is_some()
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    pub fn get_by_bounds<'a>(
        &'a self,
        assets: &'a assets::Assets,
        bounds: Bounds,
    ) -> impl Iterator<Item = (usize, &'a Entity)> {
        match bounds {
            Bounds::Logic(bounds) => {
                let iter = bounds
                    .to_grid_space(Self::LOGIC_GRID_SIZE)
                    .into_iter_points()
                    .filter_map(move |grid_point| self.logic_grid_index.get(&grid_point))
                    .flatten()
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .map(|(_, &id)| (id, &self.entity_metas[id].entity))
                    .filter(move |(_, entity)| {
                        let spec = &assets.entity_specs[entity.spec_id];
                        let entity_bounds =
                            aabb2(entity.position, entity.position + spec.logic_size);
                        bounds.intersects(entity_bounds)
                    });
                itertools::Either::Right(iter)
            }
            Bounds::View(bounds) => {
                let iter = bounds
                    .to_grid_space(Self::VIEW_GRID_SIZE)
                    .into_iter_points()
                    .filter_map(move |grid_point| self.view_grid_index.get(&grid_point))
                    .flatten()
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .map(|(_, &id)| (id, &self.entity_metas[id].entity))
                    .filter(move |(_, entity)| {
                        let spec = &assets.entity_specs[entity.spec_id];
                        let entity_bounds =
                            aabb2(entity.position, entity.position) + spec.view_size;
                        bounds.intersects(entity_bounds)
                    });
                itertools::Either::Left(iter)
            }
        }
    }
}
