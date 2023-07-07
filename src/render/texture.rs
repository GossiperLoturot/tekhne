use crate::model;
use ahash::AHashMap;
use glam::*;

#[derive(Debug)]
pub struct TextureResource {
    texcoords: AHashMap<model::ResourceKind, IVec2>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl TextureResource {
    const SIZE: u32 = 1024;
    const GRID: u32 = 32;

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let resource_kinds = [
            model::ResourceKind::SurfaceDirt,
            model::ResourceKind::SurfaceGrass,
            model::ResourceKind::SurfaceGravel,
            model::ResourceKind::SurfaceSand,
            model::ResourceKind::SurfaceStone,
        ];

        let size_per_grid = Self::SIZE / Self::GRID;
        let mut atlas = image::DynamicImage::new_rgba8(Self::SIZE, Self::SIZE);
        let mut texcoords = AHashMap::new();

        for (i, resource_kind) in resource_kinds.into_iter().enumerate() {
            if Self::GRID * Self::GRID < i as u32 {
                panic!("Atlas texture size is too small!")
            }

            let (x, y) = ((i as u32 % Self::GRID), i as u32 / Self::GRID);

            let texture = resource_kind
                .load_dynamic_image()
                .expect(&format!("failed to loading a image {:?}", resource_kind))
                .resize(
                    size_per_grid,
                    size_per_grid,
                    image::imageops::FilterType::Triangle,
                );
            image::imageops::replace(
                &mut atlas,
                &texture,
                (x * size_per_grid) as i64,
                (y * size_per_grid) as i64,
            );

            texcoords.insert(resource_kind, IVec2::new(x as i32, y as i32));
        }

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
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            atlas
                .as_rgba8()
                .expect("failet to format atlas map to rgba8"),
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[Self::GRID as f32]),
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

    pub fn texcoord(&self, resource_kind: model::ResourceKind) -> IVec2 {
        self.texcoords
            .get(&resource_kind)
            .expect(&format!(
                "not registered a resource kind {:?}",
                resource_kind
            ))
            .clone()
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
