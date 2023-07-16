use crate::model::*;
use ahash::AHashMap;
use glam::*;

#[derive(Debug)]
pub struct IUnitTextureResource {
    texcoords: AHashMap<IUnitKind, IVec2>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl IUnitTextureResource {
    const SIZE: u32 = 1024;
    const UNIT_SIZE: u32 = 32;

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let grid_x = Self::SIZE / Self::UNIT_SIZE;
        let grid_y = Self::SIZE / (Self::UNIT_SIZE * 2);
        let mip_level_count = Self::UNIT_SIZE.ilog2();

        let mut atlas = vec![];
        let mut texcoords = AHashMap::new();

        for mip_level in 0..mip_level_count {
            let size = Self::SIZE >> mip_level;
            atlas.push(image::DynamicImage::new_rgba8(size, size));
        }

        for (i, kind) in IUnitKind::entry().into_iter().enumerate() {
            if grid_x * grid_y < i as u32 {
                panic!("Atlas texture size is too small!");
            }

            let (x, y) = (i as u32 % grid_x, i as u32 / grid_x);

            if let Some(texture) = kind.top_texture() {
                for mip_level in 0..mip_level_count {
                    let unit_size = Self::UNIT_SIZE >> mip_level;
                    let texture = texture.resize_exact(
                        unit_size,
                        unit_size,
                        image::imageops::FilterType::Triangle,
                    );
                    image::imageops::replace(
                        &mut atlas[mip_level as usize],
                        &texture,
                        (unit_size * x) as i64,
                        (unit_size * y) as i64,
                    );
                }
            }

            if let Some(texture) = kind.side_texture() {
                for mip_level in 0..mip_level_count {
                    let unit_size = Self::UNIT_SIZE >> mip_level;
                    let texture = texture.resize_exact(
                        unit_size,
                        unit_size,
                        image::imageops::FilterType::Triangle,
                    );
                    image::imageops::replace(
                        &mut atlas[mip_level as usize],
                        &texture,
                        (unit_size * x) as i64,
                        (unit_size * y + unit_size) as i64,
                    );
                }
            }

            texcoords.insert(kind, IVec2::new(x as i32, y as i32));
        }

        let atlas_data = atlas
            .into_iter()
            .map(|atlas| atlas.to_rgba8().to_vec())
            .flatten()
            .collect::<Vec<_>>();

        use wgpu::util::DeviceExt;
        let texture = device.create_texture_with_data(
            &queue,
            &wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: Self::SIZE,
                    height: Self::SIZE,
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

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[grid_x as f32, grid_y as f32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
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
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            texcoords,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn get_texcoord(&self, kind: &IUnitKind) -> Option<IVec2> {
        self.texcoords.get(kind).cloned()
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
