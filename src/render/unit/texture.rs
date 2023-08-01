use super::unit_kind::TextureOption;
use crate::model::*;
use ahash::AHashMap;
use glam::*;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use wgpu::util::DeviceExt;

pub struct AtlasTexcoord {
    pub page: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct AtlasResult {
    pub block_size: u32,
    pub size_per_block: u32,
    pub mip_level_count: u32,
    pub page_count: u32,
    pub texcoords: AHashMap<UnitKind, AtlasTexcoord>,
    pub atlas_set: Vec<Vec<image::DynamicImage>>,
}

pub fn atlas_packing(
    block_size: u32,
    size_per_block: u32,
    max_page_count: u32,
    items: &[UnitKind],
) -> Option<AtlasResult> {
    let mut rects = rectangle_pack::GroupedRectsToPlace::<_, ()>::new();
    for &item in items {
        let (width, height) = item.texture_size();
        let rect = rectangle_pack::RectToInsert::new(width + 2, height + 2, 1);
        rects.push_rect(item, None, rect);
    }

    let mut target_bin = BTreeMap::new();
    target_bin.insert(
        (),
        rectangle_pack::TargetBin::new(block_size, block_size, max_page_count),
    );

    let locations = rectangle_pack::pack_rects(
        &rects,
        &mut target_bin,
        &rectangle_pack::volume_heuristic,
        &rectangle_pack::contains_smallest_box,
    )
    .ok()?;

    let mut texcoords = AHashMap::new();

    let page_count = locations
        .packed_locations()
        .iter()
        .map(|(_, (_, location))| location.z())
        .max()?
        + 1;
    let mip_level_count = size_per_block.ilog2();
    let mut atlas_set = vec![];
    for _ in 0..page_count {
        let mut atlas = vec![];
        for mip_level in 0..mip_level_count {
            let size = (size_per_block >> mip_level) * block_size;
            atlas.push(image::DynamicImage::new_rgba8(size, size));
        }
        atlas_set.push(atlas);
    }

    for (&item, (_, location)) in locations.packed_locations() {
        println!("{:?} {:?}", item, location);

        let texcoord = AtlasTexcoord {
            page: location.z(),
            x: (location.x() + 1) as f32 / block_size as f32,
            y: (location.y() + 1) as f32 / block_size as f32,
            width: (location.width() - 2) as f32 / block_size as f32,
            height: (location.height() - 2) as f32 / block_size as f32,
        };
        texcoords.insert(item, texcoord);

        let texture = item.texture()?;
        let (width, height) = item.texture_size();
        let texture_option = item.texture_option();

        let mut dilation = image::DynamicImage::new_rgba8(
            size_per_block * (width + 2),
            size_per_block * (height + 2),
        );
        match texture_option {
            TextureOption::Single => image::imageops::replace(
                &mut dilation,
                &texture,
                size_per_block as i64,
                size_per_block as i64,
            ),
            TextureOption::Continuous => {
                for x in -1..=1 {
                    for y in -1..=1 {
                        image::imageops::replace(
                            &mut dilation,
                            &texture,
                            size_per_block as i64 + (size_per_block * width) as i64 * x,
                            size_per_block as i64 + (size_per_block * height) as i64 * y,
                        );
                    }
                }
            }
        }

        let atlas = atlas_set.get_mut(location.z() as usize)?;
        for mip_level in 0..mip_level_count {
            let size_per_block = size_per_block >> mip_level;
            let mip_map = dilation.resize(
                size_per_block * (width + 2),
                size_per_block * (height + 2),
                image::imageops::FilterType::Triangle,
            );

            let target = atlas.get_mut(mip_level as usize)?;
            image::imageops::replace(
                target,
                &mip_map,
                (size_per_block * location.x()) as i64,
                (size_per_block * location.y()) as i64,
            );
        }
    }

    Some(AtlasResult {
        block_size,
        size_per_block,
        mip_level_count,
        page_count,
        texcoords,
        atlas_set,
    })
}

pub struct UnitTextureResource {
    page_count: u32,
    texcoords: AHashMap<UnitKind, AtlasTexcoord>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl UnitTextureResource {
    const BLOCK_SIZE: u32 = 32;
    const SIZE_PER_BLOCK: u32 = 32;
    const MAX_PAGE_COUNT: u32 = 255;

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let atlas_result = atlas_packing(
            Self::BLOCK_SIZE,
            Self::SIZE_PER_BLOCK,
            Self::MAX_PAGE_COUNT,
            &UnitKind::iter().collect::<Vec<_>>(),
        )
        .expect("failed to generate atlas");

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
        for atlas in atlas_result.atlas_set {
            let atlas_data = atlas
                .iter()
                .flat_map(|mip_map| mip_map.to_rgba8().to_vec())
                .collect::<Vec<_>>();

            let texture = device.create_texture_with_data(
                queue,
                &wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: Self::SIZE_PER_BLOCK * Self::BLOCK_SIZE,
                        height: Self::SIZE_PER_BLOCK * Self::BLOCK_SIZE,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: atlas_result.mip_level_count,
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
            page_count: atlas_result.page_count,
            texcoords: atlas_result.texcoords,
            bind_group_layout,
            bind_groups,
        }
    }

    pub fn page_count(&self) -> u32 {
        self.page_count
    }

    pub fn texcoord(&self, unit_kind: &UnitKind) -> Option<&AtlasTexcoord> {
        self.texcoords.get(unit_kind)
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self, page: u32) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(page as usize)
    }
}
