#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
    Tree,
    Rock,
}

impl ResourceKind {
    pub fn entry() -> &'static [ResourceKind] {
        &[
            ResourceKind::SurfaceDirt,
            ResourceKind::SurfaceGrass,
            ResourceKind::SurfaceGravel,
            ResourceKind::SurfaceSand,
            ResourceKind::SurfaceStone,
            ResourceKind::Tree,
            ResourceKind::Rock,
        ]
    }

    pub fn load_dynamic_image(&self) -> Option<image::DynamicImage> {
        #[rustfmt::skip]
        let bytes: &[u8] = match self {
            ResourceKind::SurfaceDirt => include_bytes!("../../assets/textures/SurfaceDirt.jpg"),
            ResourceKind::SurfaceGrass => include_bytes!("../../assets/textures/SurfaceGrass.jpg"),
            ResourceKind::SurfaceGravel => include_bytes!("../../assets/textures/SurfaceGravel.jpg"),
            ResourceKind::SurfaceSand => include_bytes!("../../assets/textures/SurfaceSand.jpg"),
            ResourceKind::SurfaceStone => include_bytes!("../../assets/textures/SurfaceStone.jpg"),
            ResourceKind::Tree => include_bytes!("../../assets/textures/Frame.png"),
            ResourceKind::Rock => include_bytes!("../../assets/textures/Frame.png"),
        };

        image::load_from_memory(bytes).map(|image| image).ok()
    }

    pub fn scale(&self) -> f32 {
        match self {
            ResourceKind::Tree => 4.0,
            _ => 1.0,
        }
    }

    pub fn breakable(&self) -> bool {
        match self {
            ResourceKind::Tree => true,
            ResourceKind::Rock => true,
            _ => false,
        }
    }
}
