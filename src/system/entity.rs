//! エンティティシステムに関するモジュール

use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;
use slab::Slab;

/// ワールド上のエンティティの操作を行うシステム
#[derive(Default)]
pub struct EntitySystem {
    entities: Slab<Entity>,
    index_table: AHashMap<IVec3, AHashSet<usize>>,
}

impl EntitySystem {
    /// 近傍探索の効率化のための空間分割サイズ
    const GRID_SIZE: f32 = 32.0;

    /// エンティティを追加する。
    ///
    /// 成功した場合は`self`内での識別子を返す。
    /// 識別子は削除後に再利用されることに注意しなければならない。
    pub fn insert(&mut self, entity: Entity) -> usize {
        let id = self.entities.vacant_key();

        let grid_aabb = (entity.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb.iter().for_each(|index| {
            self.index_table.entry(index).or_default().insert(id);
        });

        self.entities.insert(entity)
    }

    /// エンティティを削除する。
    ///
    /// エンティティが存在し、削除に成功した場合は`Some(Block)`を返す。
    /// 逆にエンティティが存在せず、削除に失敗した場合は`None`を返す。
    pub fn remove(&mut self, id: usize) -> Option<Entity> {
        let entity = self.entities.try_remove(id);

        if let Some(entity) = entity {
            let grid_aabb = (entity.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().remove(&id);
            });

            Some(entity)
        } else {
            None
        }
    }

    /// ワールド上のエンティティを取得する。
    ///
    /// ワールド上にエンティティが存在する場合は`Some(&Block)`を返す。
    /// 逆にエンティティが存在しない場合は`None`を返す。
    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// ワールド上のエンティティを範囲指定によって取得する。
    ///
    /// ワールド上においてAABBと交差するすべてのエンティティの識別子と参照を返す。
    pub fn get_by_aabb(&self, aabb: Aabb3A) -> Vec<(usize, &Entity)> {
        let grid_aabb = (aabb / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|index| self.index_table.get(&index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|&id| self.entities.get(id).map(|entity| (id, entity)))
            .filter(|(_, entity)| aabb.intersect(entity.aabb()))
            .collect::<Vec<_>>()
    }
}
