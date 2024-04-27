//! ベースシステムの機能に関するモジュール

use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::aabb::*;
use crate::assets;

#[derive(Clone)]
pub struct Base {
    pub spec_id: usize,
    pub position: IVec2,
}

impl Base {
    /// 新しいベースを作成する。
    #[inline]
    pub fn new(spec_id: usize, position: IVec2) -> Self {
        Self { spec_id, position }
    }
}

struct BaseMeta {
    base: Base,
    grid_index_rev: (IVec2, usize),
    global_index_rev: IVec2,
}

/// ベースシステムの機能
pub struct BaseStorage {
    assets: std::rc::Rc<assets::Assets>,
    base_metas: Slab<BaseMeta>,
    grid_index: HashMap<IVec2, Slab<usize>>,
    global_index: HashMap<IVec2, usize>,
}

impl BaseStorage {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    const VOLUME_THRESHOLD: i32 = 256;

    #[inline]
    pub fn new(assets: std::rc::Rc<assets::Assets>) -> Self {
        Self {
            assets,
            base_metas: Default::default(),
            grid_index: Default::default(),
            global_index: Default::default(),
        }
    }

    /// ベースを追加し、識別子を返す。
    pub fn insert(&mut self, base: Base) -> Option<usize> {
        // 重複の回避
        let rect = iaabb2(base.position, base.position + IVec2::ONE);
        if self.has_internal_by_rect(rect) {
            return None;
        }

        let base_id = self.base_metas.vacant_key();

        // インデクスを構築 (1)
        let grid_point = base.position.to_grid_space(Self::GRID_SIZE);
        let id = self
            .grid_index
            .entry(grid_point)
            .or_default()
            .insert(base_id);
        let grid_index_rev = (grid_point, id);

        // インデクスを構築 (2)
        self.global_index.insert(base.position, base_id);
        let global_index_rev = base.position;

        self.base_metas.insert(BaseMeta {
            base,
            grid_index_rev,
            global_index_rev,
        });
        Some(base_id)
    }

    /// ベースを削除し、そのベースを返す。
    pub fn remove(&mut self, id: usize) -> Option<Base> {
        let BaseMeta {
            base,
            grid_index_rev,
            global_index_rev,
        } = self.base_metas.try_remove(id)?;

        // インデクスを破棄 (1)
        let (grid_point, id) = grid_index_rev;
        self.grid_index.get_mut(&grid_point).unwrap().remove(id);

        // インデクスを破棄 (2)
        self.global_index.remove(&global_index_rev);

        Some(base)
    }

    /// 指定した識別子に対応するベースの参照を返す。
    #[inline]
    pub fn get(&self, id: usize) -> Option<&Base> {
        self.base_metas.get(id).map(|base_meta| &base_meta.base)
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn has_rendering_by_rect(&self, rect: Aabb2) -> bool {
        self.get_rendering_by_rect(rect).next().is_some()
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。
    pub fn get_internal_by_rect(&self, rect: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        if rect.volume() <= Self::VOLUME_THRESHOLD {
            // 指定した範囲に存在するベースの識別子と参照を返す。狭い範囲で効果的。
            let iter = rect
                .into_iter_points()
                .filter_map(move |position| self.global_index.get(&position))
                .map(|&id| (id, &self.base_metas[id].base));
            itertools::Either::Right(iter)
        } else {
            // 指定した範囲に存在するベースの識別子と参照を返す。広い範囲で効果的。
            let iter = rect
                .to_grid_space(Self::GRID_SIZE)
                .into_iter_points()
                .filter_map(move |grid_point| self.grid_index.get(&grid_point))
                .flatten()
                .map(|(_, &id)| (id, &self.base_metas[id].base))
                .filter(move |(_, base)| {
                    let base_rect = iaabb2(base.position, base.position + IVec2::ONE);
                    rect.intersects(base_rect)
                });
            itertools::Either::Left(iter)
        }
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn has_internal_by_rect(&self, rect: IAabb2) -> bool {
        self.get_internal_by_rect(rect).next().is_some()
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。
    pub fn get_rendering_by_rect(&self, rect: Aabb2) -> impl Iterator<Item = (usize, &Base)> {
        let rect = rect.trunc_over().as_iaabb2();

        if rect.volume() <= Self::VOLUME_THRESHOLD {
            // 指定した範囲に存在するベースの識別子と参照を返す。狭い範囲で効果的。
            let iter = rect
                .into_iter_points()
                .filter_map(move |position| self.global_index.get(&position))
                .map(|&id| (id, &self.base_metas[id].base));
            itertools::Either::Right(iter)
        } else {
            // 指定した範囲に存在するベースの識別子と参照を返す。広い範囲で効果的。
            let iter = rect
                .to_grid_space(Self::GRID_SIZE)
                .into_iter_points()
                .filter_map(move |grid_point| self.grid_index.get(&grid_point))
                .flatten()
                .map(|(_, &id)| (id, &self.base_metas[id].base))
                .filter(move |(_, base)| {
                    let base_rect = iaabb2(base.position, base.position + IVec2::ONE);
                    rect.intersects(base_rect)
                });
            itertools::Either::Left(iter)
        }
    }
}
