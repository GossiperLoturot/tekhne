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

struct BlockMeta {
    block: Block,
    grid_index_rev: Vec<(IVec2, usize)>,
    global_index_rev: IAabb2,
}

/// ブロックシステムの機能
pub struct BlockSystem {
    block_metas: Slab<BlockMeta>,
    grid_index: HashMap<IVec2, Slab<usize>>,
    global_index: HashMap<IVec2, usize>,
}

impl BlockSystem {
    /// 近傍探索のための空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            block_metas: Default::default(),
            grid_index: Default::default(),
            global_index: Default::default(),
        }
    }

    /// ブロックを追加し、識別子を返す。
    pub fn insert(&mut self, assets: &assets::Assets, block: Block) -> Option<usize> {
        let spec = &assets.block_specs[block.spec_id];
        let bounds = iaabb2(block.position, block.position + spec.size);

        // 重複の回避
        if self.contains_from_bounds(assets, bounds) {
            return None;
        }

        let block_id = self.block_metas.vacant_key();

        // インデクスを構築 (1)
        let grid_bounds = bounds.to_grid(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min, grid_bounds.max);
        let grid_index_rev = itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .map(|grid| {
                let id = self.grid_index.entry(grid).or_default().insert(block_id);
                (grid, id)
            })
            .collect::<Vec<_>>();

        // インデクスを構築 (2)
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .for_each(|position| {
                self.global_index.insert(position, block_id);
            });
        let global_index_rev = bounds;

        self.block_metas.insert(BlockMeta {
            block,
            grid_index_rev,
            global_index_rev,
        });
        Some(block_id)
    }

    /// ブロックを削除し、そのブロックを返す。
    pub fn remove(&mut self, assets: &assets::Assets, id: usize) -> Option<Block> {
        let BlockMeta {
            block,
            grid_index_rev,
            global_index_rev,
        } = self.block_metas.try_remove(id)?;

        // インデクスを破棄 (1)
        grid_index_rev.into_iter().for_each(|(grid, id)| {
            self.grid_index.get_mut(&grid).unwrap().remove(id);
        });

        // インデクスを破棄 (2)
        let bounds = global_index_rev;
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .for_each(|position| {
                self.global_index.remove(&position);
            });

        Some(block)
    }

    /// 指定した識別子に対応するブロックの参照を返す。
    #[inline]
    pub fn get(&self, id: usize) -> Option<&Block> {
        self.block_metas.get(id).map(|block_meta| &block_meta.block)
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn contains_from_bounds(&self, assets: &assets::Assets, bounds: IAabb2) -> bool {
        self.get_from_bounds(assets, bounds).next().is_some()
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

    /// 指定した範囲に存在するブロックの識別子と参照を返す。狭い範囲で効果的。
    fn get_from_bounds_small(&self, bounds: IAabb2) -> impl Iterator<Item = (usize, &Block)> {
        let (min, max) = (bounds.min, bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |position| self.global_index.get(&position))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|&id| (id, &self.block_metas[id].block))
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。広い範囲で効果的。
    fn get_from_bounds_large<'a>(
        &'a self,
        assets: &'a assets::Assets,
        bounds: IAabb2,
    ) -> impl Iterator<Item = (usize, &'a Block)> {
        let grid_bounds = bounds.to_grid(Self::GRID_SIZE);
        let (min, max) = (grid_bounds.min, grid_bounds.max);
        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
            .map(|(x, y)| ivec2(x, y))
            .filter_map(move |grid| self.grid_index.get(&grid))
            .flatten()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|(_, &id)| (id, &self.block_metas[id].block))
            .filter(move |(_, block)| {
                let spec = &assets.block_specs[block.spec_id];
                let block_bounds = iaabb2(block.position, block.position + spec.size);
                bounds.intersects(block_bounds)
            })
    }
}
