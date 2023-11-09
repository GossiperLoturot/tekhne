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
        if self.contains_from_bounds(bounds) {
            return None;
        }

        let base_id = self.base_metas.vacant_key();

        // インデクスを構築 (1)
        let grid = base.position.div_euclid(IVec2::splat(Self::GRID_SIZE));
        let id = self.grid_index.entry(grid).or_default().insert(base_id);
        let grid_index_rev = (grid, id);

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
        let (grid, id) = grid_index_rev;
        self.grid_index.get_mut(&grid).unwrap().remove(id);

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
    pub fn contains_from_bounds(&self, bounds: IAabb2) -> bool {
        self.get_from_bounds(bounds).next().is_some()
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。
    #[inline]
    pub fn get_from_bounds(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        const VOLUME_THRESHOLD: i32 = 256;
        if bounds.volume() <= VOLUME_THRESHOLD {
            itertools::Either::Right(self.get_from_bounds_small(bounds))
        } else {
            itertools::Either::Left(self.get_from_bounds_large(bounds))
        }
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。狭い範囲で効果的。
    fn get_from_bounds_small(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |position| self.global_index.get(&position))
            .map(|&id| (id, &self.base_metas[id].base))
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。広い範囲で効果的。
    fn get_from_bounds_large(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        let grid_bounds = bounds.to_grid(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min, grid_bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |grid| self.grid_index.get(&grid))
            .flatten()
            .map(|(_, &id)| (id, &self.base_metas[id].base))
            .filter(move |(_, base)| {
                let base_bounds = iaabb2(base.position, base.position + IVec2::ONE);
                bounds.intersects(base_bounds)
            })
    }
}
