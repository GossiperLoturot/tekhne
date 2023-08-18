//! ブロックとエンティティの描写を目的とする追加情報に関するモジュール

use super::{texture::UnitAtlasOption, UnitVertex};
use crate::model::*;
use strum::EnumIter;

/// 3Dモデルの種類
pub enum UnitModelItem {
    ITop1x1,
    IBottom1x1,
    IBottom2x2,
    IBottom4x2,
    IBillboard4x6,
    Billboard1x2,
}

impl UnitModelItem {
    /// 3Dモデルの頂点データを返す。
    #[inline]
    #[rustfmt::skip]
    pub fn vertices(&self) -> &[UnitVertex] {
        match self {
            UnitModelItem::ITop1x1 => &[
                UnitVertex { position: [0.0, 0.0, 1.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [1.0, 0.0, 1.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [1.0, 1.0, 1.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 1.0, 1.0], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::IBottom1x1 => &[
                UnitVertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                UnitVertex { position: [1.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                UnitVertex { position: [1.0, 1.0, 0.1], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 1.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::IBottom2x2 => &[
                UnitVertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                UnitVertex { position: [2.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                UnitVertex { position: [2.0, 2.0, 0.1], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 2.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::IBottom4x2 => &[
                UnitVertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                UnitVertex { position: [4.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                UnitVertex { position: [4.0, 2.0, 0.1], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 2.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::IBillboard4x6 => &[
                UnitVertex { position: [-1.5, 0.0, 0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 2.5, 0.0, 0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 2.5, 0.0, 6.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-1.5, 0.0, 6.0], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::Billboard1x2 => &[
                UnitVertex { position: [-0.5, 0.0, 0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, 0.0, 0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5, 0.0, 2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5, 0.0, 2.0], texcoord: [0.0, 0.0] },
            ],
        }
    }

    /// 3Dモデルのインデクスデータを返す。
    #[inline]
    pub fn indices(&self) -> &[u32] {
        match self {
            UnitModelItem::ITop1x1 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::IBottom1x1 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::Billboard1x2 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::IBottom2x2 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::IBottom4x2 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::IBillboard4x6 => &[0, 1, 2, 2, 3, 0],
        }
    }
}

impl From<BlockKind> for UnitModelItem {
    #[inline]
    fn from(value: BlockKind) -> Self {
        match value {
            BlockKind::SurfaceDirt => Self::ITop1x1,
            BlockKind::SurfaceGrass => Self::ITop1x1,
            BlockKind::SurfaceGravel => Self::ITop1x1,
            BlockKind::SurfaceSand => Self::ITop1x1,
            BlockKind::SurfaceStone => Self::ITop1x1,
            BlockKind::MixGrass => Self::IBottom1x1,
            BlockKind::Dandelion => Self::IBottom1x1,
            BlockKind::FallenBranch => Self::IBottom1x1,
            BlockKind::FallenLeaves => Self::IBottom1x1,
            BlockKind::MixPebbles => Self::IBottom1x1,
            BlockKind::OakTree => Self::IBillboard4x6,
            BlockKind::BirchTree => Self::IBillboard4x6,
            BlockKind::DyingTree => Self::IBillboard4x6,
            BlockKind::FallenTree => Self::IBottom4x2,
            BlockKind::MixRock => Self::IBottom2x2,
        }
    }
}

impl From<EntityKind> for UnitModelItem {
    #[inline]
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Player => Self::Billboard1x2,
        }
    }
}

/// テクスチャの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum UnitTextureItem {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
    MixGrass,
    Dandelion,
    FallenBranch,
    FallenLeaves,
    MixPebbles,
    Player,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl From<BlockKind> for UnitTextureItem {
    #[inline]
    fn from(value: BlockKind) -> Self {
        match value {
            BlockKind::SurfaceDirt => Self::SurfaceDirt,
            BlockKind::SurfaceGrass => Self::SurfaceGrass,
            BlockKind::SurfaceGravel => Self::SurfaceGravel,
            BlockKind::SurfaceSand => Self::SurfaceSand,
            BlockKind::SurfaceStone => Self::SurfaceStone,
            BlockKind::MixGrass => Self::MixGrass,
            BlockKind::Dandelion => Self::Dandelion,
            BlockKind::FallenBranch => Self::FallenBranch,
            BlockKind::FallenLeaves => Self::FallenLeaves,
            BlockKind::MixPebbles => Self::MixPebbles,
            BlockKind::OakTree => Self::OakTree,
            BlockKind::BirchTree => Self::BirchTree,
            BlockKind::DyingTree => Self::DyingTree,
            BlockKind::FallenTree => Self::FallenTree,
            BlockKind::MixRock => Self::MixRock,
        }
    }
}

impl From<EntityKind> for UnitTextureItem {
    #[inline]
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Player => Self::Player,
        }
    }
}

impl UnitTextureItem {
    /// テクスチャのデータを返す。
    ///
    /// このデータはRGBA8の形式でレイアウトされる。
    #[inline]
    pub fn texture(&self) -> image::ImageResult<image::DynamicImage> {
        let bytes: &[u8] = match self {
            Self::SurfaceDirt => include_bytes!("../../../assets/textures/surface_dirt.png"),
            Self::SurfaceGrass => include_bytes!("../../../assets/textures/surface_grass.png"),
            Self::SurfaceGravel => include_bytes!("../../../assets/textures/surface_gravel.png"),
            Self::SurfaceSand => include_bytes!("../../../assets/textures/surface_sand.png"),
            Self::SurfaceStone => include_bytes!("../../../assets/textures/surface_stone.png"),
            Self::MixGrass => include_bytes!("../../../assets/textures/mix_grass.png"),
            Self::Dandelion => include_bytes!("../../../assets/textures/dandelion.png"),
            Self::FallenBranch => include_bytes!("../../../assets/textures/fallen_branch.png"),
            Self::FallenLeaves => include_bytes!("../../../assets/textures/fallen_leaves.png"),
            Self::MixPebbles => include_bytes!("../../../assets/textures/mix_pebbles.png"),
            Self::OakTree => include_bytes!("../../../assets/textures/oak_tree.png"),
            Self::BirchTree => include_bytes!("../../../assets/textures/birch_tree.png"),
            Self::DyingTree => include_bytes!("../../../assets/textures/dying_tree.png"),
            Self::FallenTree => include_bytes!("../../../assets/textures/fallen_tree.png"),
            Self::MixRock => include_bytes!("../../../assets/textures/mix_rock.png"),
            Self::Player => include_bytes!("../../../assets/textures/frame.png"),
        };

        image::load_from_memory(bytes)
    }

    /// ミップマップ生成時に用いられるテクスチャの大きさを返す。
    ///
    /// この大きさはミップマップ生成時に用いられる。正しい大きさを指定した場合は
    /// テクスチャが不自然にボケることなく描写される。
    #[inline]
    pub fn block_size(&self) -> (u32, u32) {
        match self {
            Self::SurfaceDirt => (1, 1),
            Self::SurfaceGrass => (1, 1),
            Self::SurfaceGravel => (1, 1),
            Self::SurfaceSand => (1, 1),
            Self::SurfaceStone => (1, 1),
            Self::MixGrass => (1, 1),
            Self::Dandelion => (1, 1),
            Self::FallenBranch => (1, 1),
            Self::FallenLeaves => (1, 1),
            Self::MixPebbles => (1, 1),
            Self::OakTree => (4, 6),
            Self::BirchTree => (4, 6),
            Self::DyingTree => (4, 6),
            Self::FallenTree => (4, 2),
            Self::MixRock => (2, 2),
            Self::Player => (1, 2),
        }
    }

    /// ミップマップ生成時に用いられるテクスチャの種類を返す。
    ///
    /// 詳細は[`UnitAtlasOption`]に記述される。
    #[inline]
    pub fn atlas_option(&self) -> UnitAtlasOption {
        match self {
            Self::SurfaceDirt => UnitAtlasOption::Continuous,
            Self::SurfaceGrass => UnitAtlasOption::Continuous,
            Self::SurfaceGravel => UnitAtlasOption::Continuous,
            Self::SurfaceSand => UnitAtlasOption::Continuous,
            Self::SurfaceStone => UnitAtlasOption::Continuous,
            Self::MixGrass => UnitAtlasOption::Single,
            Self::Dandelion => UnitAtlasOption::Single,
            Self::FallenBranch => UnitAtlasOption::Single,
            Self::FallenLeaves => UnitAtlasOption::Single,
            Self::MixPebbles => UnitAtlasOption::Single,
            Self::OakTree => UnitAtlasOption::Single,
            Self::BirchTree => UnitAtlasOption::Single,
            Self::DyingTree => UnitAtlasOption::Single,
            Self::FallenTree => UnitAtlasOption::Single,
            Self::MixRock => UnitAtlasOption::Single,
            Self::Player => UnitAtlasOption::Single,
        }
    }
}
