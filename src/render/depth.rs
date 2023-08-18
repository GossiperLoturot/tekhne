//! デプスマップに関するモジュール

/// デプスマップの作成と保持を行うリソース
pub struct DepthResource {
    view: wgpu::TextureView,
}

impl DepthResource {
    /// デプスマップに使用するテクスチャのフォーマット[`wgpu::TextureFormat`]
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// 新しいリソースを作成する。
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { view }
    }

    /// デプスマップに使用するテクスチャのビュー[`wgpu::TextureView`]を返す。
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}
