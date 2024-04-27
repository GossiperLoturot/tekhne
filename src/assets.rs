use std::{
    fs::File,
    path::{Path, PathBuf},
};

use glam::*;

use crate::aabb::*;

pub struct BaseSpec {
    pub id: usize,
    pub label: String,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub struct BlockSpec {
    pub id: usize,
    pub label: String,
    pub internal_size: IVec2,
    pub rendering_size: Aabb2,
    pub z_along_y: bool,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub struct EntitySpec {
    pub id: usize,
    pub label: String,
    pub internal_size: Vec2,
    pub rendering_size: Aabb2,
    pub z_along_y: bool,
    pub texture_path: PathBuf,
    pub texture_mip_option: image_atlas::AtlasEntryMipOption,
}

pub enum GenerationSpec {
    FillBase {
        id: usize,
        base_spec_id: usize,
    },
    RandomBase {
        id: usize,
        base_spec_id: usize,
        probability: f32,
    },
    RandomBlock {
        id: usize,
        block_spec_id: usize,
        probability: f32,
    },
}

pub struct PlayerSpec {
    pub id: usize,
    pub label: String,
    pub entity_spec_id: usize,
    pub texture_path: PathBuf,
}

pub struct Assets {
    pub base_specs: Vec<BaseSpec>,
    pub entity_specs: Vec<EntitySpec>,
    pub block_specs: Vec<BlockSpec>,
    pub generation_specs: Vec<GenerationSpec>,
    pub player_specs: Vec<PlayerSpec>,
}

impl Assets {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Vec2In {
            x: f32,
            y: f32,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct IVec2In {
            x: i32,
            y: i32,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Aabb2In {
            min: Vec2In,
            max: Vec2In,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BaseSpecIn {
            label: String,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BlockSpecIn {
            label: String,
            internal_size: IVec2In,
            rendering_size: Aabb2In,
            z_along_y: bool,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct EntitySpecIn {
            label: String,
            internal_size: Vec2In,
            rendering_size: Aabb2In,
            z_along_y: bool,
            texture_path: String,
            texture_mip_option: String,
        }

        #[derive(serde::Deserialize)]
        #[serde(tag = "mode", rename_all = "camelCase")]
        enum GenerationSpecIn {
            #[serde(rename_all = "camelCase")]
            FillBase { base_spec_label: String },
            #[serde(rename_all = "camelCase")]
            RandomBase {
                base_spec_label: String,
                probability: f32,
            },
            #[serde(rename_all = "camelCase")]
            RandomBlock {
                block_spec_label: String,
                probability: f32,
            },
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct PlayerSpecIn {
            label: String,
            entity_spec_label: String,
            texture_path: String,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct AssetsIn {
            base_specs: Vec<BaseSpecIn>,
            block_specs: Vec<BlockSpecIn>,
            entity_specs: Vec<EntitySpecIn>,
            generation_specs: Vec<GenerationSpecIn>,
            player_specs: Vec<PlayerSpecIn>,
        }

        let reader = File::open(path).unwrap();
        let AssetsIn {
            base_specs,
            block_specs,
            entity_specs,
            generation_specs,
            player_specs,
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
                        internal_size,
                        rendering_size,
                        z_along_y,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let internal_size = ivec2(internal_size.x, internal_size.y);
                    let rendering_size = aabb2(
                        vec2(rendering_size.min.x, rendering_size.min.y),
                        vec2(rendering_size.max.x, rendering_size.max.y),
                    );
                    let texture_path = texture_path.into();
                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    BlockSpec {
                        id,
                        label,
                        internal_size,
                        rendering_size,
                        z_along_y,
                        texture_path,
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
                        internal_size,
                        rendering_size,
                        z_along_y,
                        texture_path,
                        texture_mip_option,
                    },
                )| {
                    let internal_size = vec2(internal_size.x, internal_size.y);
                    let rendering_size = aabb2(
                        vec2(rendering_size.min.x, rendering_size.min.y),
                        vec2(rendering_size.max.x, rendering_size.max.y),
                    );
                    let texture_path = texture_path.into();
                    let texture_mip_option = match texture_mip_option.as_str() {
                        "clamp" => image_atlas::AtlasEntryMipOption::Clamp,
                        "repeat" => image_atlas::AtlasEntryMipOption::Repeat,
                        "mirror" => image_atlas::AtlasEntryMipOption::Mirror,
                        _ => unreachable!(),
                    };

                    EntitySpec {
                        id,
                        label,
                        internal_size,
                        rendering_size,
                        z_along_y,
                        texture_path,
                        texture_mip_option,
                    }
                },
            )
            .collect::<Vec<_>>();

        let generation_specs = generation_specs
            .into_iter()
            .enumerate()
            .map(|(id, spec)| match spec {
                GenerationSpecIn::FillBase { base_spec_label } => {
                    let base_spec_id = base_specs
                        .iter()
                        .find(|base_spec| base_spec.label == base_spec_label)
                        .unwrap()
                        .id;

                    GenerationSpec::FillBase { id, base_spec_id }
                }
                GenerationSpecIn::RandomBase {
                    base_spec_label,
                    probability,
                } => {
                    let base_spec_id = base_specs
                        .iter()
                        .find(|base_spec| base_spec.label == base_spec_label)
                        .unwrap()
                        .id;

                    GenerationSpec::RandomBase {
                        id,
                        base_spec_id,
                        probability,
                    }
                }
                GenerationSpecIn::RandomBlock {
                    block_spec_label,
                    probability,
                } => {
                    let block_spec_id = block_specs
                        .iter()
                        .find(|block_spec| block_spec.label == block_spec_label)
                        .unwrap()
                        .id;

                    GenerationSpec::RandomBlock {
                        id,
                        block_spec_id,
                        probability,
                    }
                }
            })
            .collect::<Vec<_>>();

        let player_specs = player_specs
            .into_iter()
            .enumerate()
            .map(
                |(
                    id,
                    PlayerSpecIn {
                        label,
                        entity_spec_label,
                        texture_path,
                    },
                )| {
                    let entity_spec_id = entity_specs
                        .iter()
                        .find(|entity_spec| entity_spec.label == entity_spec_label)
                        .unwrap()
                        .id;
                    let texture_path = texture_path.into();

                    PlayerSpec {
                        id,
                        label,
                        entity_spec_id,
                        texture_path,
                    }
                },
            )
            .collect::<Vec<_>>();

        Self {
            base_specs,
            block_specs,
            entity_specs,
            generation_specs,
            player_specs,
        }
    }
}
