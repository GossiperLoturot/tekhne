use std::{
    fs::File,
    path::{Path, PathBuf},
};

use glam::*;

pub enum YAxis {
    Y,
    YZ,
}

pub struct BaseSpec {
    pub id: usize,
    pub label: String,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub struct BlockSpec {
    pub id: usize,
    pub label: String,
    pub size: IVec2,
    pub y_axis: YAxis,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub struct EntitySpec {
    pub id: usize,
    pub label: String,
    pub size: Vec2,
    pub y_axis: YAxis,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub enum GenerationSpec {
    FillBase {
        id: usize,
        base_spec_id: usize,
    },
    RandomBlock {
        id: usize,
        block_spec_id: usize,
        probability: f32,
    },
}

pub struct Assets {
    pub base_specs: Vec<BaseSpec>,
    pub entity_specs: Vec<EntitySpec>,
    pub block_specs: Vec<BlockSpec>,
    pub generation_specs: Vec<GenerationSpec>,
}

impl Assets {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BaseSpecIn {
            label: String,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BlockSpecIn {
            label: String,
            x_size: i32,
            y_size: i32,
            y_axis: String,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct EntitySpecIn {
            label: String,
            x_size: f32,
            y_size: f32,
            y_axis: String,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(tag = "mode", rename_all = "camelCase")]
        enum GenerationSpecIn {
            #[serde(rename_all = "camelCase")]
            RandomBlock {
                block_spec_label: String,
                probability: f32,
            },
            #[serde(rename_all = "camelCase")]
            FillBase { base_spec_label: String },
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct AssetsIn {
            base_specs: Vec<BaseSpecIn>,
            block_specs: Vec<BlockSpecIn>,
            entity_specs: Vec<EntitySpecIn>,
            generation_specs: Vec<GenerationSpecIn>,
        }

        let reader = File::open(path).unwrap();
        let AssetsIn {
            base_specs,
            block_specs,
            entity_specs,
            generation_specs,
        } = serde_json::from_reader(reader).unwrap();

        let base_specs = base_specs
            .into_iter()
            .enumerate()
            .map(
                |(
                    id,
                    BaseSpecIn {
                        label,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    BaseSpec {
                        id,
                        label,
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
                        x_size,
                        y_size,
                        y_axis,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let y_axis = match y_axis.as_str() {
                        "y" => YAxis::Y,
                        "yz" => YAxis::YZ,
                        _ => unreachable!(),
                    };

                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    BlockSpec {
                        id,
                        label,
                        size: ivec2(x_size, y_size),
                        y_axis,
                        texture_path: texture_path.into(),
                        texture_mip_option,
                    }
                },
            )
            .collect::<Vec<_>>();

        let entity_specs = entity_specs
            .into_iter()
            .enumerate()
            .map(
                |(
                    id,
                    EntitySpecIn {
                        label,
                        x_size,
                        y_size,
                        y_axis,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let y_axis = match y_axis.as_str() {
                        "y" => YAxis::Y,
                        "yz" => YAxis::YZ,
                        _ => unreachable!(),
                    };

                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    EntitySpec {
                        id,
                        label,
                        size: vec2(x_size, y_size),
                        y_axis,
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
                GenerationSpecIn::FillBase { base_spec_label } => {
                    let (base_spec_id, _) = base_specs
                        .iter()
                        .enumerate()
                        .find(|(_, base_spec)| base_spec.label == base_spec_label)
                        .unwrap();

                    GenerationSpec::FillBase { id, base_spec_id }
                }
            })
            .collect::<Vec<_>>();

        Self {
            base_specs,
            block_specs,
            entity_specs,
            generation_specs,
        }
    }
}
