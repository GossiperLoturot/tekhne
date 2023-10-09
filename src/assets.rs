use std::path::PathBuf;

use glam::*;

pub struct EntitySpec {
    pub id: usize,
    pub size: Vec2,
    pub z_index: f32,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub struct BlockSpec {
    pub id: usize,
    pub size: IVec2,
    pub z_index: f32,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub enum GenerationSpec {
    RandomBlock {
        block_spec_id: usize,
        probability: f32,
    },
    FillBlock {
        block_spec_id: usize,
    },
}

pub struct Assets {
    entity_specs: Vec<EntitySpec>,
    block_specs: Vec<BlockSpec>,
    generation_specs: Vec<GenerationSpec>,
}

impl Assets {
    /// TODO: load from external storage
    pub fn new() -> Self {
        let entity_specs = vec![EntitySpec {
            id: 0,
            size: vec2(1.0, 2.0),
            z_index: 50.0,
            texture_path: "assets/textures/frame.png".into(),
            texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
        }];

        let block_specs = vec![
            BlockSpec {
                id: 0,
                size: ivec2(1, 1),
                z_index: 0.0,
                texture_path: "assets/textures/surface_grass.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Repeat,
            },
            BlockSpec {
                id: 1,
                size: ivec2(1, 1),
                z_index: 10.0,
                texture_path: "assets/textures/mix_grass.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 2,
                size: ivec2(1, 1),
                z_index: 10.0,
                texture_path: "assets/textures/dandelion.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 3,
                size: ivec2(1, 1),
                z_index: 10.0,
                texture_path: "assets/textures/fallen_branch.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 4,
                size: ivec2(1, 1),
                z_index: 10.0,
                texture_path: "assets/textures/fallen_leaves.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 5,
                size: ivec2(1, 1),
                z_index: 10.0,
                texture_path: "assets/textures/mix_pebbles.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 6,
                size: ivec2(4, 6),
                z_index: 100.0,
                texture_path: "assets/textures/oak_tree.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 7,
                size: ivec2(4, 6),
                z_index: 100.0,
                texture_path: "assets/textures/birch_tree.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 8,
                size: ivec2(4, 6),
                z_index: 100.0,
                texture_path: "assets/textures/dying_tree.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 9,
                size: ivec2(4, 2),
                z_index: 100.0,
                texture_path: "assets/textures/fallen_tree.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
            BlockSpec {
                id: 10,
                size: ivec2(2, 2),
                z_index: 100.0,
                texture_path: "assets/textures/mix_rock.png".into(),
                texture_mip_option: image_atlas::AtlasEntryMipOption::Clamp,
            },
        ];

        let generation_specs = vec![
            GenerationSpec::RandomBlock {
                block_spec_id: 1,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 2,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 3,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 4,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 5,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 6,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 7,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 8,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 9,
                probability: 0.01,
            },
            GenerationSpec::RandomBlock {
                block_spec_id: 10,
                probability: 0.01,
            },
            GenerationSpec::FillBlock { block_spec_id: 0 },
        ];

        Self {
            entity_specs,
            block_specs,
            generation_specs,
        }
    }

    #[inline]
    pub fn entity_specs(&self) -> &[EntitySpec] {
        &self.entity_specs
    }

    #[inline]
    pub fn block_specs(&self) -> &[BlockSpec] {
        &self.block_specs
    }

    #[inline]
    pub fn generation_specs(&self) -> &[GenerationSpec] {
        &self.generation_specs
    }
}
