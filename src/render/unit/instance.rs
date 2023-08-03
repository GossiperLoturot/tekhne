use super::{extract::UnitShapeItem, shape::UnitShapeResource, texture::UnitAtlasResource};
use crate::service::Service;
use ahash::AHashMap;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UnitInstance {
    pub position: [f32; 3],
    pub texcoord: [f32; 4],
}

impl UnitInstance {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x4];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub struct UnitInstanceGroup {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

pub struct UnitInstanceResource {
    instance_groups: AHashMap<(u32, UnitShapeItem), UnitInstanceGroup>,
}

impl UnitInstanceResource {
    pub fn new(
        device: &wgpu::Device,
        atlas_resource: &UnitAtlasResource,
        shape_resource: &UnitShapeResource,
    ) -> Self {
        let mut instance_groups = AHashMap::new();
        for page in 0..atlas_resource.page_count() {
            for &shape in shape_resource.shapes() {
                let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: device.limits().max_buffer_size,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

                instance_groups.insert(
                    (page, shape),
                    UnitInstanceGroup {
                        instance_buffer,
                        instance_count: 0,
                    },
                );
            }
        }

        Self { instance_groups }
    }

    pub fn pre_draw(
        &mut self,
        queue: &wgpu::Queue,
        service: &Service,
        atlas_resource: &UnitAtlasResource,
    ) {
        if let Some(camera) = service.camera.get_camera() {
            let view_aabb = camera.view_aabb();

            let units = service
                .unit
                .get_units(view_aabb)
                .into_iter()
                .map(|unit| (unit.position, unit.kind));

            let iunits = service
                .iunit
                .get_iunits(view_aabb.as_iaabb3())
                .into_iter()
                .map(|iunit| (iunit.position.as_vec3a(), iunit.kind));

            let mut instances = AHashMap::new();
            for (position, unit_kind) in Iterator::chain(units, iunits) {
                let texcoord = atlas_resource
                    .texcoord(&unit_kind.into())
                    .unwrap_or_else(|| panic!("not registered unit kind {:?}", &unit_kind));

                let page = texcoord.page;
                let shape = unit_kind.into();

                let position = [position.x, position.y, position.z];
                let texcoord = [texcoord.x, texcoord.y, texcoord.width, texcoord.height];

                instances
                    .entry((page, shape))
                    .or_insert(vec![])
                    .push(UnitInstance { position, texcoord });
            }

            for (id, instances) in instances {
                let instance_group = self
                    .instance_groups
                    .get_mut(&id)
                    .expect("failed to get instance group");

                instance_group.instance_count = instances.len() as u32;
                queue.write_buffer(
                    &instance_group.instance_buffer,
                    0,
                    bytemuck::cast_slice(&instances),
                );
            }
        }
    }

    pub fn instance_group(&self, page: u32, shape: &UnitShapeItem) -> Option<&UnitInstanceGroup> {
        self.instance_groups.get(&(page, shape.clone()))
    }
}
