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
    index: HashMap<IVec2, Slab<usize>>,
    rev_index: Slab<usize>,
}

impl BaseSystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            bases: Default::default(),
            index: Default::default(),
            rev_index: Default::default(),
        }
    }

    /// ベースを追加し、識別子を返す。
    pub fn insert(&mut self, base: Base) -> usize {
        let id = self.bases.vacant_key();

        // インデクスを構築
        let point = base
            .position
            .div_euclid(ivec2(Self::GRID_SIZE, Self::GRID_SIZE));
        let idx_id = self.index.entry(point).or_default().insert(id);
        self.rev_index.insert(idx_id);

        self.bases.insert(base)
    }

    /// ベースを削除し、そのベースを返す。
    pub fn remove(&mut self, id: usize) -> Option<Base> {
        let base = self.bases.try_remove(id)?;

        // インデクスを破棄
        let idx_id = self.rev_index.remove(id);
        let point = base
            .position
            .div_euclid(ivec2(Self::GRID_SIZE, Self::GRID_SIZE));
        self.index.get_mut(&point).unwrap().remove(idx_id);

        Some(base)
    }

    /// 指定した識別子に対応するベースの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Base> {
        self.bases.get(id)
    }

    /// 指定した範囲に存在するベースの識別子と参照を返す。
    #[inline]
    pub fn get_from_area(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Base)> {
        let grid_bounds = bounds.div_euclid_i32(Self::GRID_SIZE);
        let min = grid_bounds.min;
        let max = grid_bounds.max;
        let iter = (min.x..=max.x).flat_map(move |x| (min.y..=max.y).map(move |y| ivec2(x, y)));

        iter.filter_map(move |point| self.index.get(&point))
            .flatten()
            .map(|(_, &id)| (id, &self.bases[id]))
    }
}
