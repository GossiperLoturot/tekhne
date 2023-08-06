use crate::render::unit::model::UnitTextureItem;
use ahash::AHashMap;
use glam::*;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use wgpu::util::DeviceExt;

pub enum UnitAtlasOption {
    Single,
    Continuous,
}

pub struct UnitAtlasTexcoord {
    pub page: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct UnitTextureResource {
    texcoords: AHashMap<UnitTextureItem, UnitAtlasTexcoord>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl UnitTextureResource {
    const MAX_PAGE_COUNT: u32 = 255;
    const BLOCK_SIZE: u32 = 64;
    const SIZE_PER_BLOCK: u32 = 32;

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut rects = rectangle_pack::GroupedRectsToPlace::<_, ()>::new();
        for item in UnitTextureItem::iter() {
            let (width, height) = item.block_size();
            rects.push_rect(
                item,
                None,
                rectangle_pack::RectToInsert::new(width + 1, height + 1, 1),
            )
        }

        let mut target_bin = BTreeMap::new();
        target_bin.insert(
            (),
            rectangle_pack::TargetBin::new(
                Self::BLOCK_SIZE,
                Self::BLOCK_SIZE,
                Self::MAX_PAGE_COUNT,
            ),
        );

        let locations = rectangle_pack::pack_rects(
            &rects,
            &mut target_bin,
            &rectangle_pack::volume_heuristic,
            &rectangle_pack::contains_smallest_box,
        )
        .expect("failed to pack atlas layout");

        let page_count = locations
            .packed_locations()
            .iter()
            .map(|(_, (_, location))| location.z())
            .max()
            .expect("failed to compute atlas page count")
            + 1;
        let size = Self::SIZE_PER_BLOCK * Self::BLOCK_SIZE;
        let mip_level_count = Self::SIZE_PER_BLOCK.ilog2();
        let mut atlas_set = vec![];
        for _ in 0..page_count {
            let mut atlas = vec![];
            for mip_level in 0..mip_level_count {
                let size = size >> mip_level;
                atlas.push(image::DynamicImage::new_rgba8(size, size));
            }
            atlas_set.push(atlas);
        }

        let mut texcoords = AHashMap::new();

        for (item, (_, location)) in locations.packed_locations() {
            let texture = item.texture().expect("failed to load texture");
            let (width, height) = item.block_size();

            let mut dilation = image::DynamicImage::new_rgba8(
                Self::SIZE_PER_BLOCK * (width + 1),
                Self::SIZE_PER_BLOCK * (height + 1),
            );

            match item.atlas_option() {
                UnitAtlasOption::Single => {
                    image::imageops::replace(
                        &mut dilation,
                        &texture,
                        (Self::SIZE_PER_BLOCK / 2) as i64,
                        (Self::SIZE_PER_BLOCK / 2) as i64,
                    );
                }
                UnitAtlasOption::Continuous => {
                    for x in -1..=1 {
                        for y in -1..=1 {
                            image::imageops::replace(
                                &mut dilation,
                                &texture,
                                (Self::SIZE_PER_BLOCK / 2) as i64
                                    + (Self::SIZE_PER_BLOCK * width) as i64 * x,
                                (Self::SIZE_PER_BLOCK / 2) as i64
                                    + (Self::SIZE_PER_BLOCK * height) as i64 * y,
                            )
                        }
                    }
                }
            }

            for mip_level in 0..mip_level_count {
                let size_per_block = Self::SIZE_PER_BLOCK >> mip_level;
                let mip_map = dilation.resize(
                    size_per_block * (width + 1),
                    size_per_block * (height + 1),
                    image::imageops::Triangle,
                );

                image::imageops::replace(
                    &mut atlas_set[location.z() as usize][mip_level as usize],
                    &mip_map,
                    (size_per_block * location.x()) as i64,
                    (size_per_block * location.y()) as i64,
                );
            }

            texcoords.insert(
                item.clone(),
                UnitAtlasTexcoord {
                    page: location.z(),
                    x: (Self::SIZE_PER_BLOCK * location.x() + Self::SIZE_PER_BLOCK / 2) as f32
                        / size as f32,
                    y: (Self::SIZE_PER_BLOCK * location.y() + Self::SIZE_PER_BLOCK / 2) as f32
                        / size as f32,
                    width: (Self::SIZE_PER_BLOCK * width) as f32 / size as f32,
                    height: (Self::SIZE_PER_BLOCK * height) as f32 / size as f32,
                },
            );
        }

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let mut bind_groups = vec![];
        for atlas in atlas_set {
            let atlas_data = atlas
                .into_iter()
                .flat_map(|mip_map| mip_map.to_rgba8().to_vec())
                .collect::<Vec<_>>();

            let texture = device.create_texture_with_data(
                queue,
                &wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: size,
                        height: size,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                },
                &atlas_data,
            );
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_sampler),
                    },
                ],
            });
            bind_groups.push(bind_group);
        }

        Self {
            texcoords,
            bind_group_layout,
            bind_groups,
        }
    }

    pub fn page_count(&self) -> u32 {
        self.bind_groups.len() as u32
    }

    pub fn texcoord(&self, item: &UnitTextureItem) -> Option<&UnitAtlasTexcoord> {
        self.texcoords.get(item)
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self, page: u32) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(page as usize)
    }
}
