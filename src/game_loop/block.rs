//! ブロックシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::assets;

pub struct Block {
    pub spec_id: usize,
    pub position: IVec2,
}

impl Block {
    /// 新しいブロックを作成する。
    #[inline]
    pub fn new(spec_id: usize, position: IVec2) -> Self {
        Self { spec_id, position }
    }
}

/// ブロックシステムの機能
pub struct BlockSystem {
    blocks: Slab<Block>,
    index: HashMap<(IVec2, usize), Slab<usize>>,
    rev_index: Slab<usize>,
}

impl BlockSystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            blocks: Default::default(),
            index: Default::default(),
            rev_index: Default::default(),
        }
    }

    /// ブロックを追加し、識別子を返す。
    pub fn insert(&mut self, assets: &assets::Assets, block: Block) -> usize {
        let id = self.blocks.vacant_key();

        let spec = &assets.block_specs[block.spec_id];

        // インデクスを構築
        let point = block
            .position
            .div_euclid(ivec2(Self::GRID_SIZE, Self::GRID_SIZE));
        let idx_id = self
            .index
            .entry((point, spec.layer_id))
            .or_default()
            .insert(id);
        self.rev_index.insert(idx_id);

        self.blocks.insert(block)
    }

    /// ブロックを削除し、そのブロックを返す。
    pub fn remove(&mut self, assets: &assets::Assets, id: usize) -> Option<Block> {
        let block = self.blocks.try_remove(id)?;

        let spec = &assets.block_specs[block.spec_id];

        // インデクスを破棄
        let idx_id = self.rev_index.remove(id);
        let point = block
            .position
            .div_euclid(ivec2(Self::GRID_SIZE, Self::GRID_SIZE));
        self.index
            .get_mut(&(point, spec.layer_id))
            .unwrap()
            .remove(idx_id);

        Some(block)
    }

    /// 指定した識別子に対応するブロックの参照を返す。
    pub fn get(&self, id: usize) -> Option<&Block> {
        self.blocks.get(id)
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。
    #[inline]
    pub fn get_from_area<'a>(
        &'a self,
        bounds: IAabb2,
        layer_id: usize,
    ) -> impl Iterator<Item = (usize, &'a Block)> {
        let grid_bounds = bounds.div_euclid_i32(Self::GRID_SIZE);
        let min = grid_bounds.min;
        let max = grid_bounds.max;
        let iter = (min.x..=max.x).flat_map(move |x| (min.y..=max.y).map(move |y| ivec2(x, y)));

        iter.filter_map(move |point| self.index.get(&(point, layer_id)))
            .flatten()
            .map(|(_, &id)| (id, &self.blocks[id]))
    }
}
