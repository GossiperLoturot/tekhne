//! ブロックシステムの機能に関するモジュール

use aabb::*;
use ahash::HashMap;
use glam::*;
use slab::Slab;

use crate::game_loop;

pub enum Bounds {
    Logic(IAabb2),
    View(Aabb2),
}

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
    logic_grid_index_rev: Vec<(IVec2, usize)>,
    view_grid_index_rev: Vec<(IVec2, usize)>,
}

/// ブロックシステムの機能
pub struct BlockStorage {
    block_metas: Slab<BlockMeta>,
    logic_grid_index: HashMap<IVec2, Slab<usize>>,
    view_grid_index: HashMap<IVec2, Slab<usize>>,
}

impl BlockStorage {
    /// 近傍探索のための空間分割サイズ
    const LOGIC_GRID_SIZE: i32 = 32;

    /// 近傍探索のための空間分割サイズ
    const VIEW_GRID_SIZE: f32 = 32.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            block_metas: Default::default(),
            logic_grid_index: Default::default(),
            view_grid_index: Default::default(),
        }
    }

    /// ブロックを追加し、識別子を返す。
    pub fn insert(&mut self, cx: &game_loop::InputContext, block: Block) -> Option<usize> {
        let spec = &cx.assets.block_specs[block.spec_id];

        // 重複の回避
        let bounds = iaabb2(block.position, block.position + spec.logic_size);
        if self.exists_by_bounds(cx, Bounds::Logic(bounds)) {
            return None;
        }

        let block_id = self.block_metas.vacant_key();

        // インデクスを構築 (1)
        let bounds = iaabb2(block.position, block.position + spec.logic_size);
        let logic_grid_index_rev = bounds
            .to_grid_space(Self::LOGIC_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .logic_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(block_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        // インデクスを構築 (2)
        let bounds = iaabb2(block.position, block.position).as_aabb2() + spec.view_size;
        let view_grid_index_rev = bounds
            .to_grid_space(Self::VIEW_GRID_SIZE)
            .into_iter_points()
            .map(|grid_point| {
                let id = self
                    .view_grid_index
                    .entry(grid_point)
                    .or_default()
                    .insert(block_id);
                (grid_point, id)
            })
            .collect::<Vec<_>>();

        self.block_metas.insert(BlockMeta {
            block,
            logic_grid_index_rev,
            view_grid_index_rev,
        });
        Some(block_id)
    }

    /// ブロックを削除し、そのブロックを返す。
    pub fn remove(&mut self, cx: &game_loop::InputContext, id: usize) -> Option<Block> {
        let BlockMeta {
            block,
            logic_grid_index_rev,
            view_grid_index_rev,
        } = self.block_metas.try_remove(id)?;

        // インデクスを破棄 (1)
        logic_grid_index_rev
            .into_iter()
            .for_each(|(grid_point, id)| {
                self.logic_grid_index
                    .get_mut(&grid_point)
                    .unwrap()
                    .remove(id);
            });

        // インデクスを破棄 (2)
        view_grid_index_rev
            .into_iter()
            .for_each(|(grid_point, id)| {
                self.view_grid_index
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
    pub fn exists_by_bounds(&self, cx: &game_loop::InputContext, bounds: Bounds) -> bool {
        self.get_by_bounds(cx, bounds).next().is_some()
    }

    /// 指定した範囲に存在するブロックの識別子と参照を返す。
    pub fn get_by_bounds<'a>(
        &'a self,
        cx: &'a game_loop::InputContext,
        bounds: Bounds,
    ) -> impl Iterator<Item = (usize, &'a Block)> {
        match bounds {
            Bounds::Logic(bounds) => {
                let iter = bounds
                    .to_grid_space(Self::LOGIC_GRID_SIZE)
                    .into_iter_points()
                    .filter_map(move |grid_point| self.logic_grid_index.get(&grid_point))
                    .flatten()
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .map(|(_, &id)| (id, &self.block_metas[id].block))
                    .filter(move |(_, block)| {
                        let spec = &cx.assets.block_specs[block.spec_id];
                        let block_bounds = iaabb2(block.position, block.position + spec.logic_size);
                        bounds.intersects(block_bounds)
                    });
                itertools::Either::Right(iter)
            }
            Bounds::View(bounds) => {
                let iter = bounds
                    .to_grid_space(Self::VIEW_GRID_SIZE)
                    .into_iter_points()
                    .filter_map(move |grid_point| self.view_grid_index.get(&grid_point))
                    .flatten()
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .map(|(_, &id)| (id, &self.block_metas[id].block))
                    .filter(move |(_, block)| {
                        let spec = &cx.assets.block_specs[block.spec_id];
                        let block_bounds =
                            iaabb2(block.position, block.position).as_aabb2() + spec.view_size;
                        bounds.intersects(block_bounds)
                    });
                itertools::Either::Left(iter)
            }
        }
    }
}
