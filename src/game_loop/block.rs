//! ブロックシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::assets;

pub struct Block {
    pub spec_id: usize,
    pub position: IVec2,
    pub z_random: u8,
}

impl Block {
    /// 新しいブロックを作成する。
    #[inline]
    pub fn new(spec_id: usize, position: IVec2, z_random: u8) -> Self {
        Self {
            spec_id,
            position,
            z_random,
        }
    }
}

/// ブロックシステムの機能
pub struct BlockSystem {
    blocks: Slab<Block>,
    grid_index: HashMap<IVec2, Slab<usize>>,
    grid_index_rev: Slab<usize>,
    global_index: HashMap<IVec2, usize>,
}

impl BlockSystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            blocks: Default::default(),
            grid_index: Default::default(),
            grid_index_rev: Default::default(),
            global_index: Default::default(),
        }
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn contains(&self, assets: &assets::Assets, bounds: IAabb2) -> bool {
        self.get_from_bounds(assets, bounds).next().is_some()
    }

    /// ブロックを追加し、識別子を返す。
    pub fn insert(&mut self, assets: &assets::Assets, block: Block) -> Option<usize> {
        let spec = &assets.block_specs[block.spec_id];
        let bounds = iaabb2(block.position, block.position + spec.size);

        // 重複の回避
        if self.contains(assets, bounds) {
            return None;
        }

        let id = self.blocks.vacant_key();

        // インデクスを構築 (1)
        let grid_point = block.position.div_euclid(IVec2::splat(Self::GRID_SIZE));
        let idx = self.grid_index.entry(grid_point).or_default().insert(id);
        self.grid_index_rev.insert(idx);

        // インデクスを構築 (2)
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .for_each(|position| {
                self.global_index.insert(position, id);
            });

        self.blocks.insert(block);
        Some(id)
    }

    /// ブロックを削除し、そのブロックを返す。
    pub fn remove(&mut self, assets: &assets::Assets, id: usize) -> Option<Block> {
        let block = self.blocks.try_remove(id)?;

        let spec = &assets.block_specs[block.spec_id];
        let bounds = iaabb2(block.position, block.position + spec.size - IVec2::ONE);

        // インデクスを破棄 (1)
        let idx = self.grid_index_rev.remove(id);
        let grid_point = block.position.div_euclid(IVec2::splat(Self::GRID_SIZE));
        self.grid_index.get_mut(&grid_point).unwrap().remove(idx);

        // インデクスを破棄 (2)
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .for_each(|position| {
                self.global_index.insert(position, id);
            });

        Some(block)
    }

    /// 指定した識別子に対応するブロックの参照を返す。
    #[inline]
    pub fn get(&self, id: usize) -> Option<&Block> {
        self.blocks.get(id)
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。
    #[inline]
    pub fn get_from_bounds<'a>(
        &'a self,
        assets: &'a assets::Assets,
        bounds: IAabb2,
    ) -> impl Iterator<Item = (usize, &'a Block)> {
        const VOLUME_THRESHOLD: i32 = 256;
        if bounds.volume() <= VOLUME_THRESHOLD {
            itertools::Either::Right(self.get_from_bounds_small(bounds))
        } else {
            itertools::Either::Left(self.get_from_bounds_large(assets, bounds))
        }
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。広い範囲で効果的。
    fn get_from_bounds_large<'a>(
        &'a self,
        assets: &'a assets::Assets,
        bounds: IAabb2,
    ) -> impl Iterator<Item = (usize, &'a Block)> {
        let grid_bounds =
            iaabb2(bounds.min, bounds.max - IVec2::ONE).div_euclid_i32(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min - IVec2::ONE, grid_bounds.max + IVec2::ONE);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |grid_point| self.grid_index.get(&grid_point))
            .flatten()
            .map(|(_, &id)| (id, &self.blocks[id]))
            .filter(move |(_, block)| {
                let spec = &assets.block_specs[block.spec_id];
                let self_bounds = iaabb2(block.position, block.position + spec.size);
                bounds.intersect(self_bounds)
            })
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。狭い範囲で効果的。
    fn get_from_bounds_small(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Block)> {
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |position| self.global_index.get(&position))
            .map(|&id| (id, &self.blocks[id]))
    }
}
