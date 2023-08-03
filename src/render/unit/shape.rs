use super::extract::UnitShapeItem;
use ahash::AHashMap;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UnitVertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
}

impl UnitVertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub struct UnitShapeGroup {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

pub struct UnitShapeResource {
    shape_groups: AHashMap<UnitShapeItem, UnitShapeGroup>,
}

impl UnitShapeResource {
    pub fn new(device: &wgpu::Device, items: &[UnitShapeItem]) -> Self {
        let mut shape_groups = AHashMap::new();
        for &item in items {
            let vertices = item.vertices();
            let indices = item.indices();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            let shape_group = UnitShapeGroup {
                vertex_buffer,
                index_buffer,
                index_count: indices.len() as u32,
            };
            shape_groups.insert(item, shape_group);
        }

        Self { shape_groups }
    }

    pub fn shapes(&self) -> Vec<&UnitShapeItem> {
        self.shape_groups.keys().collect()
    }

    pub fn shape_group(&self, item: &UnitShapeItem) -> Option<&UnitShapeGroup> {
        self.shape_groups.get(item)
    }
}
