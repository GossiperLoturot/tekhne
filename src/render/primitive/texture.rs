//! プリミティブのアトラスマップ生成に関するモジュール

use super::model::TextureItem;
use ahash::AHashMap;
use glam::*;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use wgpu::util::DeviceExt;

/// アトラスマップに配置するテクスチャの大きさ(最小単位)
pub struct BlockSize(pub u32, pub u32);

/// ミップマップ生成の方式
///
/// [`AtlasOption::Continuous`]は連続的(シームレス)なテクスチャに適した処理、
/// 逆に[`AtlasOption::Single`]はそうでないテクスチャに適した処理を行う。
pub enum AtlasOption {
    Single,
    Continuous,
}

/// テクスチャのアトラスマップ上での配置
pub struct AtlasTexcoord {
    pub page: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// アトラスマップの生成と配置の取得を行うリソース
pub struct AtlasResource {
    texcoords: AHashMap<TextureItem, AtlasTexcoord>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl AtlasResource {
    /// 生成するアトラスマップの最大数
    const MAX_PAGE_COUNT: u32 = 255;

    /// 一つのアトラスマップに配置する最小単位の幅(高さ)
    ///
    /// この`Block`は[`crate::model::Block`]とは異なり、
    /// アトラスマップに配置できるテクスチャの最小単位を指す。
    const BLOCK_SIZE: u32 = 64;

    /// 最小単位の大きさ
    const SIZE_PER_BLOCK: u32 = 32;

    /// 新しいリソースを作成する。
    ///
    /// アトラスマップの生成を3次元ビンパッキング問題に帰着し、
    /// ソルバーを使用して効果的なアトラスマップの配置を計算する。
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        // 配置する箱の大きさを定義する。
        let mut rects = rectangle_pack::GroupedRectsToPlace::<_, ()>::new();
        for item in TextureItem::iter() {
            let BlockSize(width, height) = item.block_size();
            rects.push_rect(
                item,
                None,
                rectangle_pack::RectToInsert::new(width + 1, height + 1, 1),
            )
        }

        // 収納する箱の大きさを定義する。
        let mut target_bin = BTreeMap::new();
        target_bin.insert(
            (),
            rectangle_pack::TargetBin::new(
                Self::BLOCK_SIZE,
                Self::BLOCK_SIZE,
                Self::MAX_PAGE_COUNT,
            ),
        );

        // 配置を計算する。
        let locations = rectangle_pack::pack_rects(
            &rects,
            &mut target_bin,
            &rectangle_pack::volume_heuristic,
            &rectangle_pack::contains_smallest_box,
        )
        .expect("failed to pack atlas layout");

        // 配置から必要なアトラスマップを作成する。
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

        // 配置をもとにアトラスマップへテクスチャを適用する。
        for (&item, (_, location)) in locations.packed_locations() {
            let texture = item.texture().expect("failed to load texture");
            let BlockSize(width, height) = item.block_size();

            // ミップマップ生成用に大きさを拡張したテクスチャを作成する
            let mut dilation = image::DynamicImage::new_rgba8(
                Self::SIZE_PER_BLOCK * (width + 1),
                Self::SIZE_PER_BLOCK * (height + 1),
            );

            match item.atlas_option() {
                // 拡張したテクスチャの中心に1つのみ配置する。
                AtlasOption::Single => {
                    image::imageops::replace(
                        &mut dilation,
                        &texture,
                        (Self::SIZE_PER_BLOCK / 2) as i64,
                        (Self::SIZE_PER_BLOCK / 2) as i64,
                    );
                }
                // 拡張したテクスチャへ格子状に9つ配置する。
                AtlasOption::Continuous => {
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

            // ミップマップのレベルに応じて大きさを拡張したテクスチャを
            // 縮小・クリップしてアトラスマップへ貼り付ける。
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

            // 描写用にテクスチャの配置を[0,1]座標空間へ変換し、保持する。
            texcoords.insert(
                item,
                AtlasTexcoord {
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

        // それぞれのアトラスマップに対してリソースを作成する。
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

    /// 生成されたアトラスマップの数を返す。
    pub fn page_count(&self) -> u32 {
        self.bind_groups.len() as u32
    }

    /// 該当するテクスチャのアトラスマップ上の配置を返す。
    pub fn texcoord(&self, item: &TextureItem) -> Option<&AtlasTexcoord> {
        self.texcoords.get(item)
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self, page: u32) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(page as usize)
    }
}
