//! エンティティシステムの機能に関するモジュール

use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::aabb::*;
use crate::assets;

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
    internal_grid_index_ref: Vec<(IVec2, usize)>,
    rendering_grid_index_ref: Vec<(IVec2, usize)>,
}

/// エンティティシステムの機能
pub struct EntityStorage {
    assets: std::rc::Rc<assets::Assets>,
    entity_metas: Slab<EntityMeta>,
    internal_grid_index: HashMap<IVec2, Slab<usize>>,
    rendering_grid_index: HashMap<IVec2, Slab<usize>>,
}

impl EntityStorage {
    /// 近傍探索のための空間分割サイズ
    const INTERNAL_GRID_SIZE: f32 = 32.0;

    /// 近傍探索のための空間分割サイズ
    const RENDERING_GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new(assets: std::rc::Rc<assets::Assets>) -> Self {
        Self {
            assets,
            entity_metas: Default::default(),
            internal_grid_index: Default::default(),
            rendering_grid_index: Default::default(),
        }
    }

    /// エンティティを追加し、識別子を返す。
    pub fn insert(&mut self, entity: Entity) -> Option<usize> {
        let spec = &self.assets.entity_specs[entity.spec_id];

        // 重複の回避
        let rect = aabb2(entity.position, entity.position + spec.internal_size);
        if self.has_rendering_by_rect(rect) {
            return None;
        }

        let entity_id = self.entity_metas.vacant_key();

        // インデクスを構築 (1)
        let rect = aabb2(entity.position, entity.position + spec.internal_size);
        let internal_grid_index_ref = rect
            .to_grid_space(Self::INTERNAL_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .internal_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(entity_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        // インデクスを構築 (2)
        let rect = entity.position + spec.rendering_size;
        let rendering_grid_index_ref = rect
            .to_grid_space(Self::RENDERING_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .rendering_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(entity_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        self.entity_metas.insert(EntityMeta {
            entity,
            internal_grid_index_ref,
            rendering_grid_index_ref,
        });
        Some(entity_id)
    }

    /// エンティティを削除し、そのエンティティを返す。
    pub fn remove(&mut self, id: usize) -> Option<Entity> {
        let EntityMeta {
            entity,
            internal_grid_index_ref,
            rendering_grid_index_ref,
        } = self.entity_metas.try_remove(id)?;

        // インデクスを破棄 (1)
        internal_grid_index_ref
            .into_iter()
            .for_each(|(grid_point, id)| {
                self.internal_grid_index
                    .get_mut(&grid_point)
                    .unwrap()
                    .remove(id);
            });

        // インデクスを破棄 (2)
        rendering_grid_index_ref
            .into_iter()
            .for_each(|(grid_point, id)| {
                self.rendering_grid_index
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
    pub fn has_internal_by_rect(&self, rect: Aabb2) -> bool {
        self.get_internal_by_rect(rect).next().is_some()
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    pub fn get_internal_by_rect(&self, rect: Aabb2) -> impl Iterator<Item = (usize, &'_ Entity)> {
        rect.to_grid_space(Self::INTERNAL_GRID_SIZE)
            .into_iter_points()
            .filter_map(move |grid_point| self.internal_grid_index.get(&grid_point))
            .flatten()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|(_, &id)| (id, &self.entity_metas[id].entity))
            .filter(move |(_, entity)| {
                let spec = &self.assets.entity_specs[entity.spec_id];
                let entity_rect = aabb2(entity.position, entity.position + spec.internal_size);
                rect.intersects(entity_rect)
            })
    }

    /// 指定した範囲にエンティティが存在するか真偽値を返す。
    #[inline]
    pub fn has_rendering_by_rect(&self, rect: Aabb2) -> bool {
        self.get_rendering_by_rect(rect).next().is_some()
    }

    /// 指定した範囲に存在するエンティティの識別子と参照を返す。
    pub fn get_rendering_by_rect(&self, rect: Aabb2) -> impl Iterator<Item = (usize, &'_ Entity)> {
        rect.to_grid_space(Self::RENDERING_GRID_SIZE)
            .into_iter_points()
            .filter_map(move |grid_point| self.rendering_grid_index.get(&grid_point))
            .flatten()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|(_, &id)| (id, &self.entity_metas[id].entity))
            .filter(move |(_, entity)| {
                let spec = &self.assets.entity_specs[entity.spec_id];
                let entity_rect = aabb2(entity.position, entity.position) + spec.rendering_size;
                rect.intersects(entity_rect)
            })
    }
}
