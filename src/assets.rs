use std::{
    fs::File,
    path::{Path, PathBuf},
};

use glam::*;

pub struct Layer {
    pub id: usize,
    pub label: String,
}

pub struct EntitySpec {
    pub id: usize,
    pub label: String,
    pub layer_id: usize,
    pub size: Vec2,
    pub z_index: f32,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub struct BlockSpec {
    pub id: usize,
    pub label: String,
    pub layer_id: usize,
    pub size: IVec2,
    pub z_index: f32,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub enum GenerationSpec {
    RandomBlock {
        id: usize,
        block_spec_id: usize,
        probability: f32,
    },
    FillBlock {
        id: usize,
        block_spec_id: usize,
    },
}

pub struct Assets {
    pub layers: Vec<Layer>,
    pub entity_specs: Vec<EntitySpec>,
    pub block_specs: Vec<BlockSpec>,
    pub generation_specs: Vec<GenerationSpec>,
}

pub static ASSETS: std::sync::OnceLock<Assets> = std::sync::OnceLock::new();

impl Assets {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct LayerIn {
            label: String,
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct EntitySpecIn {
            label: String,
            layer_label: String,
            size_x: f32,
            size_y: f32,
            z_index: f32,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct BlockSpecIn {
            label: String,
            layer_label: String,
            size_x: i32,
            size_y: i32,
            z_index: f32,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(tag = "mode", rename_all = "snake_case")]
        enum GenerationSpecIn {
            RandomBlock {
                block_spec_label: String,
                probability: f32,
            },
            FillBlock {
                block_spec_label: String,
            },
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct AssetsIn {
            layers: Vec<LayerIn>,
            entity_specs: Vec<EntitySpecIn>,
            block_specs: Vec<BlockSpecIn>,
            generation_specs: Vec<GenerationSpecIn>,
        }

        let reader = File::open(path).unwrap();
        let AssetsIn {
            layers,
            entity_specs,
            block_specs,
            generation_specs,
        } = serde_json::from_reader(reader).unwrap();

        let layers = layers
            .into_iter()
            .enumerate()
            .map(|(id, LayerIn { label })| Layer { id, label })
            .collect::<Vec<_>>();

        let entity_specs = entity_specs
            .into_iter()
            .enumerate()
            .map(
                |(
                    id,
                    EntitySpecIn {
                        label,
                        layer_label,
                        size_x,
                        size_y,
                        z_index,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let (layer_id, _) = layers
                        .iter()
                        .enumerate()
                        .find(|(_, layer)| layer.label == layer_label)
                        .unwrap();

                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    EntitySpec {
                        id,
                        label,
                        layer_id,
                        size: vec2(size_x, size_y),
                        z_index,
                        texture_path: texture_path.into(),
                        texture_mip_option,
                    }
                },
            )
            .collect::<Vec<_>>();

        let block_specs = block_specs
            .into_iter()
            .enumerate()
            .map(
                |(
                    id,
                    BlockSpecIn {
                        label,
                        layer_label,
                        size_x,
                        size_y,
                        z_index,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let (layer_id, _) = layers
                        .iter()
                        .enumerate()
                        .find(|(_, layer)| layer.label == layer_label)
                        .unwrap();

                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    BlockSpec {
                        id,
                        label,
                        layer_id,
                        size: ivec2(size_x, size_y),
                        z_index,
                        texture_path: texture_path.into(),
                        texture_mip_option,
                    }
                },
            )
            .collect::<Vec<_>>();

        let generation_specs = generation_specs
            .into_iter()
            .enumerate()
            .map(|(id, generation)| match generation {
                GenerationSpecIn::RandomBlock {
                    block_spec_label,
                    probability,
                } => {
                    let (block_spec_id, _) = block_specs
                        .iter()
                        .enumerate()
                        .find(|(_, block_spec)| block_spec.label == block_spec_label)
                        .unwrap();

                    GenerationSpec::RandomBlock {
                        id,
                        block_spec_id,
                        probability,
                    }
                }
                GenerationSpecIn::FillBlock { block_spec_label } => {
                    let (block_spec_id, _) = block_specs
                        .iter()
                        .enumerate()
                        .find(|(_, block_spec)| block_spec.label == block_spec_label)
                        .unwrap();

                    GenerationSpec::FillBlock { id, block_spec_id }
                }
            })
            .collect::<Vec<_>>();

        Self {
            layers,
            entity_specs,
            block_specs,
            generation_specs,
        }
    }
}
