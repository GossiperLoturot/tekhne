//! ブロックシステムに関するモジュール

use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;
use slab::Slab;

/// ワールド上のブロックの操作を行うシステム
#[derive(Default)]
pub struct BlockSystem {
    blocks: Slab<Block>,
    index_table: AHashMap<IVec3, AHashSet<usize>>,
}

impl BlockSystem {
    /// 近傍探索の効率化のための空間分割サイズ
    const GRID_SIZE: f32 = 32.0;

    /// ブロックを追加する。
    ///
    /// 成功した場合は`self`内での識別子を返す。
    /// 識別子は削除後に再利用されることに注意しなければならない。
    pub fn insert(&mut self, block: Block) -> usize {
        let id = self.blocks.vacant_key();

        let grid_aabb = (block.bounds().as_aabb3a() / Self::GRID_SIZE)
            .floor()
            .as_iaabb3();
        grid_aabb.iter().for_each(|index| {
            self.index_table.entry(index).or_default().insert(id);
        });

        self.blocks.insert(block)
    }

    /// ブロックを削除する。
    ///
    /// ブロックが存在し、削除に成功した場合は`Some(Block)`を返す。
    /// 逆にブロックが存在せず、削除に失敗した場合は`None`を返す。
    pub fn remove(&mut self, id: usize) -> Option<Block> {
        let block = self.blocks.try_remove(id);

        if let Some(block) = block {
            let grid_aabb = (block.bounds().as_aabb3a() / Self::GRID_SIZE)
                .floor()
                .as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().remove(&id);
            });

            Some(block)
        } else {
            None
        }
    }

    /// ワールド上のブロックを取得する。
    ///
    /// ワールド上にブロックが存在する場合は`Some(&Block)`を返す。
    /// 逆にブロックが存在しない場合は`None`を返す。
    pub fn get(&self, id: usize) -> Option<&Block> {
        self.blocks.get(id)
    }

    /// ワールド上のブロックを範囲指定によって取得する。
    ///
    /// ワールド上においてAABBと交差するすべてのブロックの識別子と参照を返す。
    pub fn get_by_aabb(&self, aabb: IAabb3) -> Vec<(usize, &Block)> {
        let grid_aabb = (aabb.as_aabb3a() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|index| self.index_table.get(&index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|&id| self.blocks.get(id).map(|block| (id, block)))
            .filter(|(_, block)| aabb.intersect(block.bounds()))
            .collect::<Vec<_>>()
    }
}
