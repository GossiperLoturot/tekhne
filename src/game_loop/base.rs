//! ベースシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

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
pub struct BaseSystem {
    base_metas: Slab<BaseMeta>,
    grid_index: HashMap<IVec2, Slab<usize>>,
    global_index: HashMap<IVec2, usize>,
}

impl BaseSystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            base_metas: Default::default(),
            grid_index: Default::default(),
            global_index: Default::default(),
        }
    }

    /// ベースを追加し、識別子を返す。
    pub fn insert(&mut self, base: Base) -> Option<usize> {
        // 重複の回避
        let bounds = iaabb2(base.position, base.position + IVec2::ONE);
        if self.contains_by_logic_bounds(bounds) {
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
    pub fn contains_by_logic_bounds(&self, bounds: IAabb2) -> bool {
        self.get_by_logic_bounds(bounds).next().is_some()
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。
    #[inline]
    pub fn get_by_logic_bounds(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        const VOLUME_THRESHOLD: i32 = 256;
        if bounds.volume() <= VOLUME_THRESHOLD {
            itertools::Either::Right(self.get_by_bounds_small(bounds))
        } else {
            itertools::Either::Left(self.get_by_bounds_large(bounds))
        }
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn contains_by_view_bounds(&self, bounds: Aabb2) -> bool {
        self.get_by_view_bounds(bounds).next().is_some()
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。
    #[inline]
    pub fn get_by_view_bounds(&self, bounds: Aabb2) -> impl Iterator<Item = (usize, &Base)> {
        self.get_by_logic_bounds(bounds.trunc_over().as_iaabb2())
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。狭い範囲で効果的。
    fn get_by_bounds_small(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        bounds
            .into_iter_points()
            .filter_map(move |position| self.global_index.get(&position))
            .map(|&id| (id, &self.base_metas[id].base))
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。広い範囲で効果的。
    fn get_by_bounds_large(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        bounds
            .to_grid_space(Self::GRID_SIZE)
            .into_iter_points()
            .filter_map(move |grid_point| self.grid_index.get(&grid_point))
            .flatten()
            .map(|(_, &id)| (id, &self.base_metas[id].base))
            .filter(move |(_, base)| {
                let base_bounds = iaabb2(base.position, base.position + IVec2::ONE);
                bounds.intersects(base_bounds)
            })
    }
}
