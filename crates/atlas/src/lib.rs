//! This library provides atlas texture builder and container.

use std::{
    collections::{BTreeMap, HashMap},
    error, fmt, hash,
};

/// A way of mip generation.
#[derive(Clone, Copy, Debug)]
pub enum AtlasLeakOption {
    None,
    Padding(u32),
    Block(u32),
}

/// A way of texture wrapping.
#[derive(Clone, Copy, Debug)]
pub enum AtlasEntryLeakOption {
    Single,
    Repeat,
}

/// An entry of texture key and path containing an atlas texture.
#[derive(Clone, Debug)]
pub struct AtlasEntry<K, I>
where
    K: Eq + hash::Hash + Clone,
    I: image::GenericImage,
{
    pub key: K,
    pub texture: I,
    pub leak: AtlasEntryLeakOption,
}

/// A descriptor for generating an atlas texture.
#[derive(Clone, Debug)]
pub struct AtlasDescriptor<'a, K, I>
where
    K: Eq + hash::Hash + Clone,
    I: image::GenericImage,
{
    pub page_count: u32,
    pub size: u32,
    pub leak: AtlasLeakOption,
    pub entries: &'a [AtlasEntry<K, I>],
}

/// Creates a new atlas texture from a given descriptor.
pub fn create_atlas<K, I, S>(
    desc: &AtlasDescriptor<K, I>,
) -> AtlasResult<AtlasTextures<K, I::Pixel, Vec<<I::Pixel as image::Pixel>::Subpixel>, S>>
where
    K: Eq + hash::Hash + Clone,
    I: image::GenericImage,
    I::Pixel: 'static,
    S: hash::BuildHasher + Default,
{
    match desc.leak {
        AtlasLeakOption::None => {
            create_atlas_with_padding(desc.page_count, desc.size, false, 0, desc.entries)
        }
        AtlasLeakOption::Padding(padding) => {
            create_atlas_with_padding(desc.page_count, desc.size, true, padding, desc.entries)
        }
        AtlasLeakOption::Block(block_size) => {
            create_atlas_with_block(desc.page_count, desc.size, block_size, desc.entries)
        }
    }
}

fn create_atlas_with_padding<K, I, S>(
    page_count: u32,
    size: u32,
    mip: bool,
    padding: u32,
    entries: &[AtlasEntry<K, I>],
) -> AtlasResult<AtlasTextures<K, I::Pixel, Vec<<I::Pixel as image::Pixel>::Subpixel>, S>>
where
    K: Eq + hash::Hash + Clone,
    I: image::GenericImage,
    I::Pixel: 'static,
    S: hash::BuildHasher + Default,
{
    if page_count == 0 {
        return Err(AtlasError::ZeroPageCount);
    }

    if !size.is_power_of_two() {
        return Err(AtlasError::InvalidSize(size));
    }

    if entries.is_empty() {
        return Err(AtlasError::ZeroEntryCount);
    }

    let mut rects = rectangle_pack::GroupedRectsToPlace::<_, ()>::new();
    for (i, entry) in entries.iter().enumerate() {
        let rect = rectangle_pack::RectToInsert::new(
            entry.texture.width() + padding * 2,
            entry.texture.height() + padding * 2,
            1,
        );
        rects.push_rect(i, None, rect);
    }

    let mut target_bins = BTreeMap::new();
    target_bins.insert((), rectangle_pack::TargetBin::new(size, size, page_count));

    let locations = rectangle_pack::pack_rects(
        &rects,
        &mut target_bins,
        &rectangle_pack::volume_heuristic,
        &rectangle_pack::contains_smallest_box,
    )?;

    let page_count = locations
        .packed_locations()
        .iter()
        .map(|(_, (_, location))| location.z())
        .max()
        .unwrap()
        + 1;

    let mut tmp_atlas_textures = vec![image::ImageBuffer::new(size, size); page_count as usize];
    let mut texcoords = HashMap::<_, _, S>::default();
    for (&i, (_, location)) in locations.packed_locations() {
        let entry = &entries[i];

        image::imageops::replace(
            &mut tmp_atlas_textures[location.z() as usize],
            &entry_with_padding(&entry.texture, padding, entry.leak),
            location.x() as i64,
            location.y() as i64,
        );

        let texcoord = Texcoord {
            page: location.z(),
            min_x: location.x() + padding,
            min_y: location.y() + padding,
            max_x: location.x() + padding + entry.texture.width(),
            max_y: location.y() + padding + entry.texture.height(),
            size,
        };
        texcoords.insert(entry.key.clone(), texcoord);
    }

    let mip_level_count = if mip { size.ilog2() + 1 } else { 1 };
    let mut atlas_textures = vec![];
    for atlas_texture in tmp_atlas_textures {
        let mut textures = vec![];
        for mip_level in 0..mip_level_count {
            let texture = image::imageops::resize(
                &atlas_texture,
                atlas_texture.width() >> mip_level,
                atlas_texture.height() >> mip_level,
                image::imageops::FilterType::Triangle,
            );
            textures.push(texture);
        }
        atlas_textures.push(AtlasTexture { textures });
    }

    Ok(AtlasTextures {
        atlas_textures,
        texcoords,
    })
}

fn create_atlas_with_block<K, I, S>(
    page_count: u32,
    size: u32,
    block_size: u32,
    entries: &[AtlasEntry<K, I>],
) -> AtlasResult<AtlasTextures<K, I::Pixel, Vec<<I::Pixel as image::Pixel>::Subpixel>, S>>
where
    K: Eq + hash::Hash + Clone,
    I: image::GenericImage,
    I::Pixel: 'static,
    S: hash::BuildHasher + Default,
{
    if page_count == 0 {
        return Err(AtlasError::ZeroPageCount);
    }

    if !size.is_power_of_two() {
        return Err(AtlasError::InvalidSize(size));
    }

    if !block_size.is_power_of_two() {
        return Err(AtlasError::InvalidBlockSize(block_size));
    }

    if entries.is_empty() {
        return Err(AtlasError::ZeroEntryCount);
    }

    let mut rects = rectangle_pack::GroupedRectsToPlace::<_, ()>::new();
    for (i, entry) in entries.iter().enumerate() {
        let rect = rectangle_pack::RectToInsert::new(
            ((entry.texture.width() + block_size) as f32 / block_size as f32).ceil() as u32,
            ((entry.texture.height() + block_size) as f32 / block_size as f32).ceil() as u32,
            1,
        );
        rects.push_rect(i, None, rect);
    }

    let bin_size = size / block_size;
    let mut target_bins = BTreeMap::new();
    target_bins.insert(
        (),
        rectangle_pack::TargetBin::new(bin_size, bin_size, page_count),
    );

    let locations = rectangle_pack::pack_rects(
        &rects,
        &mut target_bins,
        &rectangle_pack::volume_heuristic,
        &rectangle_pack::contains_smallest_box,
    )?;

    let page_count = locations
        .packed_locations()
        .iter()
        .map(|(_, (_, location))| location.z())
        .max()
        .unwrap()
        + 1;

    let mip_level_count = block_size.ilog2() + 1;
    let mut tmp_atlas_textures = vec![];
    for _ in 0..page_count {
        let mut atlas_texture = vec![];
        for mip_level in 0..mip_level_count {
            let size = size >> mip_level;
            let texture = image::ImageBuffer::new(size, size);
            atlas_texture.push(texture);
        }
        tmp_atlas_textures.push(atlas_texture);
    }

    let padding = block_size >> 1;
    let mut texcoords = HashMap::<_, _, S>::default();
    for (&i, (_, location)) in locations.packed_locations() {
        let entry = &entries[i];

        for mip_level in 0..mip_level_count {
            let texture = entry_with_padding(&entry.texture, padding, entry.leak);

            let texture = image::imageops::resize(
                &texture,
                texture.width() >> mip_level,
                texture.height() >> mip_level,
                image::imageops::FilterType::Triangle,
            );

            image::imageops::replace(
                &mut tmp_atlas_textures[location.z() as usize][mip_level as usize],
                &texture,
                (location.x() * block_size >> mip_level) as i64,
                (location.y() * block_size >> mip_level) as i64,
            );
        }

        let texcoord = Texcoord {
            page: location.z(),
            min_x: location.x() * block_size + padding,
            min_y: location.y() * block_size + padding,
            max_x: location.x() * block_size + padding + entry.texture.width(),
            max_y: location.y() * block_size + padding + entry.texture.height(),
            size,
        };
        texcoords.insert(entry.key.clone(), texcoord);
    }

    let mut atlas_textures = vec![];
    for atlas_texture in tmp_atlas_textures {
        let textures = atlas_texture;
        atlas_textures.push(AtlasTexture { textures })
    }

    Ok(AtlasTextures {
        atlas_textures,
        texcoords,
    })
}

fn entry_with_padding<I>(
    src: &I,
    padding: u32,
    leak: AtlasEntryLeakOption,
) -> image::ImageBuffer<I::Pixel, Vec<<I::Pixel as image::Pixel>::Subpixel>>
where
    I: image::GenericImage,
{
    match leak {
        AtlasEntryLeakOption::Single => {
            let mut target =
                image::ImageBuffer::new(src.width() + padding * 2, src.height() + padding * 2);
            image::imageops::replace(&mut target, src, padding as i64, padding as i64);
            target
        }
        AtlasEntryLeakOption::Repeat => {
            let mut target =
                image::ImageBuffer::new(src.width() + padding * 2, src.height() + padding * 2);
            for x in -1..=1 {
                for y in -1..=1 {
                    let x = padding as i32 + src.width() as i32 * x;
                    let y = padding as i32 + src.height() as i32 * y;
                    image::imageops::replace(&mut target, src, x as i64, y as i64);
                }
            }
            target
        }
    }
}

/// A result of atlas builder, which has atlas textures and texture coordinates.
pub struct AtlasTextures<K, P, Container, S>
where
    K: Eq + hash::Hash,
    P: image::Pixel,
    S: hash::BuildHasher,
{
    pub atlas_textures: Vec<AtlasTexture<P, Container>>,
    pub texcoords: HashMap<K, Texcoord, S>,
}

/// A texture array contain original and generated mip.
pub struct AtlasTexture<P, Container>
where
    P: image::Pixel,
{
    pub textures: Vec<image::ImageBuffer<P, Container>>,
}

/// A texture coordinate.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Texcoord {
    pub page: u32,
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub size: u32,
}

impl Texcoord {
    /// Returns normalized min x.
    #[inline]
    pub fn norm_min_x(&self) -> f32 {
        self.min_x as f32 / self.size as f32
    }

    /// Returns normalized min y.
    #[inline]
    pub fn norm_min_y(&self) -> f32 {
        self.min_y as f32 / self.size as f32
    }

    /// Returns normalized max x.
    #[inline]
    pub fn norm_max_x(&self) -> f32 {
        self.max_x as f32 / self.size as f32
    }

    /// Returns normalized max y.
    #[inline]
    pub fn norm_max_y(&self) -> f32 {
        self.max_y as f32 / self.size as f32
    }
}

type AtlasResult<T> = Result<T, AtlasError>;

#[derive(Debug)]
pub enum AtlasError {
    ZeroPageCount,
    InvalidSize(u32),
    InvalidBlockSize(u32),
    ZeroEntryCount,
    Packing(rectangle_pack::RectanglePackError),
}

impl fmt::Display for AtlasError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtlasError::ZeroPageCount => write!(f, "Atlas page count must be greater than zero."),
            AtlasError::InvalidSize(size) => write!(f, "Atlas size must be power of two. Actually {}.", size),
            AtlasError::InvalidBlockSize(block_size) => write!(f, "Atlas block size must be power of two. Actually {}.", block_size),
            AtlasError::ZeroEntryCount => write!(f, "Atlas entry count must be not empty."),
            AtlasError::Packing(err) => err.fmt(f),
        }
    }
}

impl error::Error for AtlasError {}

impl From<rectangle_pack::RectanglePackError> for AtlasError {
    fn from(value: rectangle_pack::RectanglePackError) -> Self {
        AtlasError::Packing(value)
    }
}
