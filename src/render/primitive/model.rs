//! プリミティブ単体に関するモジュール

use super::{
    pipeline::Vertex,
    texture::{AtlasOption, BlockSize},
};
use crate::model::*;
use strum::EnumIter;

/// 3Dモデルの種類
pub enum ModelItem {
    ITop1x1,
    IBottom1x1,
    IBottom2x2,
    IBottom4x2,
    IBillboard4x6,
    Billboard1x2,
}

impl ModelItem {
    /// 3Dモデルの頂点データを返す。
    #[inline]
    #[rustfmt::skip]
    pub fn vertices(&self) -> &[Vertex] {
        match self {
            Self::ITop1x1 => &[
                Vertex { position: [0.0, 0.0, 1.0], texcoord: [0.0, 1.0] },
                Vertex { position: [1.0, 0.0, 1.0], texcoord: [1.0, 1.0] },
                Vertex { position: [1.0, 1.0, 1.0], texcoord: [1.0, 0.0] },
                Vertex { position: [0.0, 1.0, 1.0], texcoord: [0.0, 0.0] },
            ],
            Self::IBottom1x1 => &[
                Vertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                Vertex { position: [1.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                Vertex { position: [1.0, 1.0, 0.1], texcoord: [1.0, 0.0] },
                Vertex { position: [0.0, 1.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            Self::IBottom2x2 => &[
                Vertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                Vertex { position: [2.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                Vertex { position: [2.0, 2.0, 0.1], texcoord: [1.0, 0.0] },
                Vertex { position: [0.0, 2.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            Self::IBottom4x2 => &[
                Vertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                Vertex { position: [4.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                Vertex { position: [4.0, 2.0, 0.1], texcoord: [1.0, 0.0] },
                Vertex { position: [0.0, 2.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            Self::IBillboard4x6 => &[
                Vertex { position: [-1.5, 0.0, 0.0], texcoord: [0.0, 1.0] },
                Vertex { position: [ 2.5, 0.0, 0.0], texcoord: [1.0, 1.0] },
                Vertex { position: [ 2.5, 0.0, 6.0], texcoord: [1.0, 0.0] },
                Vertex { position: [-1.5, 0.0, 6.0], texcoord: [0.0, 0.0] },
            ],
            Self::Billboard1x2 => &[
                Vertex { position: [-0.5, 0.0, 0.0], texcoord: [0.0, 1.0] },
                Vertex { position: [ 0.5, 0.0, 0.0], texcoord: [1.0, 1.0] },
                Vertex { position: [ 0.5, 0.0, 2.0], texcoord: [1.0, 0.0] },
                Vertex { position: [-0.5, 0.0, 2.0], texcoord: [0.0, 0.0] },
            ],
        }
    }

    /// 3Dモデルのインデクスデータを返す。
    #[inline]
    pub fn indices(&self) -> &[u32] {
        match self {
            Self::ITop1x1 => &[0, 1, 2, 2, 3, 0],
            Self::IBottom1x1 => &[0, 1, 2, 2, 3, 0],
            Self::Billboard1x2 => &[0, 1, 2, 2, 3, 0],
            Self::IBottom2x2 => &[0, 1, 2, 2, 3, 0],
            Self::IBottom4x2 => &[0, 1, 2, 2, 3, 0],
            Self::IBillboard4x6 => &[0, 1, 2, 2, 3, 0],
        }
    }
}

impl From<BlockKind> for ModelItem {
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

impl From<EntityKind> for ModelItem {
    #[inline]
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Player => Self::Billboard1x2,
        }
    }
}

/// テクスチャの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum TextureItem {
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

impl From<BlockKind> for TextureItem {
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

impl From<EntityKind> for TextureItem {
    #[inline]
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Player => Self::Player,
        }
    }
}

impl TextureItem {
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
    /// 詳細は[`BlockSize`]に記述される。
    #[inline]
    pub fn block_size(&self) -> BlockSize {
        match self {
            Self::SurfaceDirt => BlockSize(1, 1),
            Self::SurfaceGrass => BlockSize(1, 1),
            Self::SurfaceGravel => BlockSize(1, 1),
            Self::SurfaceSand => BlockSize(1, 1),
            Self::SurfaceStone => BlockSize(1, 1),
            Self::MixGrass => BlockSize(1, 1),
            Self::Dandelion => BlockSize(1, 1),
            Self::FallenBranch => BlockSize(1, 1),
            Self::FallenLeaves => BlockSize(1, 1),
            Self::MixPebbles => BlockSize(1, 1),
            Self::OakTree => BlockSize(4, 6),
            Self::BirchTree => BlockSize(4, 6),
            Self::DyingTree => BlockSize(4, 6),
            Self::FallenTree => BlockSize(4, 2),
            Self::MixRock => BlockSize(2, 2),
            Self::Player => BlockSize(1, 2),
        }
    }

    /// ミップマップ生成時に用いられるテクスチャの種類を返す。
    ///
    /// 詳細は[`AtlasOption`]に記述される。
    #[inline]
    pub fn atlas_option(&self) -> AtlasOption {
        match self {
            Self::SurfaceDirt => AtlasOption::Continuous,
            Self::SurfaceGrass => AtlasOption::Continuous,
            Self::SurfaceGravel => AtlasOption::Continuous,
            Self::SurfaceSand => AtlasOption::Continuous,
            Self::SurfaceStone => AtlasOption::Continuous,
            Self::MixGrass => AtlasOption::Single,
            Self::Dandelion => AtlasOption::Single,
            Self::FallenBranch => AtlasOption::Single,
            Self::FallenLeaves => AtlasOption::Single,
            Self::MixPebbles => AtlasOption::Single,
            Self::OakTree => AtlasOption::Single,
            Self::BirchTree => AtlasOption::Single,
            Self::DyingTree => AtlasOption::Single,
            Self::FallenTree => AtlasOption::Single,
            Self::MixRock => AtlasOption::Single,
            Self::Player => AtlasOption::Single,
        }
    }
}
