use ahash::RandomState;

#[test]
fn create() {
    atlas::create_atlas::<_, _, RandomState>(&atlas::AtlasDescriptor {
        page_count: 8,
        size: 512,
        leak: atlas::AtlasLeakOption::None,
        entries: &[
            atlas::AtlasEntry {
                key: "frame",
                texture: image::open("../../assets/textures/frame.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Single,
            },
            atlas::AtlasEntry {
                key: "surface_dirt",
                texture: image::open("../../assets/textures/surface_dirt.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Repeat,
            },
            atlas::AtlasEntry {
                key: "surface_grass",
                texture: image::open("../../assets/textures/surface_grass.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Repeat,
            },
        ],
    })
    .unwrap();
}

#[test]
fn create_with_padding() {
    atlas::create_atlas::<_, _, RandomState>(&atlas::AtlasDescriptor {
        page_count: 8,
        size: 512,
        leak: atlas::AtlasLeakOption::Padding(4),
        entries: &[
            atlas::AtlasEntry {
                key: "frame",
                texture: image::open("../../assets/textures/frame.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Single,
            },
            atlas::AtlasEntry {
                key: "surface_dirt",
                texture: image::open("../../assets/textures/surface_dirt.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Repeat,
            },
            atlas::AtlasEntry {
                key: "surface_grass",
                texture: image::open("../../assets/textures/surface_grass.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Repeat,
            },
        ],
    })
    .unwrap();
}

#[test]
fn create_with_block() {
    atlas::create_atlas::<_, _, RandomState>(&atlas::AtlasDescriptor {
        page_count: 8,
        size: 512,
        leak: atlas::AtlasLeakOption::Block(32),
        entries: &[
            atlas::AtlasEntry {
                key: "frame",
                texture: image::open("../../assets/textures/frame.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Single,
            },
            atlas::AtlasEntry {
                key: "surface_dirt",
                texture: image::open("../../assets/textures/surface_dirt.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Repeat,
            },
            atlas::AtlasEntry {
                key: "surface_grass",
                texture: image::open("../../assets/textures/surface_grass.png").unwrap(),
                leak: atlas::AtlasEntryLeakOption::Repeat,
            },
        ],
    })
    .unwrap();
}
