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

/// ベースシステムの機能
pub struct BaseSystem {
    bases: Slab<Base>,
    grid_index: HashMap<IVec2, Slab<usize>>,
    grid_index_rev: Slab<usize>,
    global_index: HashMap<IVec2, usize>,
}

impl BaseSystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            bases: Default::default(),
            grid_index: Default::default(),
            grid_index_rev: Default::default(),
            global_index: Default::default(),
        }
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn contains(&self, bounds: IAabb2) -> bool {
        self.get_from_bounds(bounds).next().is_some()
    }

    /// ベースを追加し、識別子を返す。
    pub fn insert(&mut self, base: Base) -> Option<usize> {
        // 重複の回避
        if self.contains(iaabb2(base.position, base.position + IVec2::ONE)) {
            return None;
        }

        let id = self.bases.vacant_key();

        // インデクスを構築 (1)
        let grid_point = base.position.div_euclid(IVec2::splat(Self::GRID_SIZE));
        let idx = self.grid_index.entry(grid_point).or_default().insert(id);
        self.grid_index_rev.insert(idx);

        // インデクスを構築 (2)
        self.global_index.insert(base.position, id);

        self.bases.insert(base);
        Some(id)
    }

    /// ベースを削除し、そのベースを返す。
    pub fn remove(&mut self, id: usize) -> Option<Base> {
        let base = self.bases.try_remove(id)?;

        // インデクスを破棄 (1)
        let idx = self.grid_index_rev.remove(id);
        let grid_point = base.position.div_euclid(IVec2::splat(Self::GRID_SIZE));
        self.grid_index.get_mut(&grid_point).unwrap().remove(idx);

        // インデクスを破棄 (2)
        self.global_index.remove(&base.position);

        Some(base)
    }

    /// 指定した識別子に対応するベースの参照を返す。
    #[inline]
    pub fn get(&self, id: usize) -> Option<&Base> {
        self.bases.get(id)
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

    /// 指定した範囲に存在するベースの識別子と参照を返す。広い範囲で効果的。
    fn get_from_bounds_large(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        let grid_bounds =
            iaabb2(bounds.min, bounds.max - IVec2::ONE).div_euclid_i32(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min - IVec2::ONE, grid_bounds.max + IVec2::ONE);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |grid_point| self.grid_index.get(&grid_point))
            .flatten()
            .map(|(_, &id)| (id, &self.bases[id]))
            .filter(move |(_, base)| {
                let self_bounds = iaabb2(base.position, base.position + IVec2::ONE);
                bounds.intersect(self_bounds)
            })
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。狭い範囲で効果的。
    fn get_from_bounds_small(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |position| self.global_index.get(&position))
            .map(|&id| (id, &self.bases[id]))
    }
}
