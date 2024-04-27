//! ブロックシステムの機能に関するモジュール

use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::aabb::*;
use crate::assets;

#[derive(Clone)]
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
    internal_grid_index_ref: Vec<(IVec2, usize)>,
    rendering_grid_index_ref: Vec<(IVec2, usize)>,
}

/// ブロックシステムの機能
pub struct BlockStorage {
    assets: std::rc::Rc<assets::Assets>,
    block_metas: Slab<BlockMeta>,
    internal_grid_index: HashMap<IVec2, Slab<usize>>,
    rendering_grid_index: HashMap<IVec2, Slab<usize>>,
}

impl BlockStorage {
    /// 近傍探索のための空間分割サイズ
    const INTERNAL_GRID_SIZE: i32 = 32;

    /// 近傍探索のための空間分割サイズ
    const RENDERING_GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new(assets: std::rc::Rc<assets::Assets>) -> Self {
        Self {
            assets,
            block_metas: Default::default(),
            internal_grid_index: Default::default(),
            rendering_grid_index: Default::default(),
        }
    }

    /// ブロックを追加し、識別子を返す。
    pub fn insert(&mut self, block: Block) -> Option<usize> {
        let spec = &self.assets.block_specs[block.spec_id];

        // 重複の回避
        let rect = iaabb2(block.position, block.position + spec.internal_size);
        if self.has_internal_by_rect(rect) {
            return None;
        }

        let block_id = self.block_metas.vacant_key();

        // インデクスを構築 (1)
        let rect = iaabb2(block.position, block.position + spec.internal_size);
        let internal_grid_index_rev = rect
            .to_grid_space(Self::INTERNAL_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .internal_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(block_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        // インデクスを構築 (2)
        let rect = iaabb2(block.position, block.position).as_aabb2() + spec.rendering_size;
        let rendering_grid_index_rev = rect
            .to_grid_space(Self::RENDERING_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .rendering_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(block_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        self.block_metas.insert(BlockMeta {
            block,
            internal_grid_index_ref: internal_grid_index_rev,
            rendering_grid_index_ref: rendering_grid_index_rev,
        });
        Some(block_id)
    }

    /// ブロックを削除し、そのブロックを返す。
    pub fn remove(&mut self, id: usize) -> Option<Block> {
        let BlockMeta {
            block,
            internal_grid_index_ref,
            rendering_grid_index_ref,
        } = self.block_metas.try_remove(id)?;

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

        Some(block)
    }

    /// 指定した識別子に対応するブロックの参照を返す。
    #[inline]
    pub fn get(&self, id: usize) -> Option<&Block> {
        self.block_metas.get(id).map(|block_meta| &block_meta.block)
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn has_internal_by_rect(&self, rect: IAabb2) -> bool {
        self.get_internal_by_rect(rect).next().is_some()
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。
    pub fn get_internal_by_rect(&self, rect: IAabb2) -> impl Iterator<Item = (usize, &'_ Block)> {
        rect.to_grid_space(Self::INTERNAL_GRID_SIZE)
            .into_iter_points()
            .filter_map(move |grid_point| self.internal_grid_index.get(&grid_point))
            .flatten()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|(_, &id)| (id, &self.block_metas[id].block))
            .filter(move |(_, block)| {
                let spec = &self.assets.block_specs[block.spec_id];
                let block_rect = iaabb2(block.position, block.position + spec.internal_size);
                rect.intersects(block_rect)
            })
    }

    /// 指定した範囲にベースが存在するか真偽値を返す。
    #[inline]
    pub fn has_rendering_by_rect(&self, rect: Aabb2) -> bool {
        self.get_rendering_by_rect(rect).next().is_some()
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。
    pub fn get_rendering_by_rect(&self, rect: Aabb2) -> impl Iterator<Item = (usize, &'_ Block)> {
        rect.to_grid_space(Self::RENDERING_GRID_SIZE)
            .into_iter_points()
            .filter_map(move |grid_point| self.rendering_grid_index.get(&grid_point))
            .flatten()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|(_, &id)| (id, &self.block_metas[id].block))
            .filter(move |(_, block)| {
                let spec = &self.assets.block_specs[block.spec_id];
                let block_rect =
                    iaabb2(block.position, block.position).as_aabb2() + spec.rendering_size;
                rect.intersects(block_rect)
            })
    }
}
