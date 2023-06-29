#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
}

impl ResourceKind {
    pub fn load_dynamic_image(&self) -> Option<image::DynamicImage> {
        #[rustfmt::skip]
        let bytes: &[u8] = match self {
            ResourceKind::SurfaceDirt => include_bytes!("../../assets/textures/SurfaceDirt.jpg"),
            ResourceKind::SurfaceGrass => include_bytes!("../../assets/textures/SurfaceGrass.jpg"),
            ResourceKind::SurfaceGravel => include_bytes!("../../assets/textures/SurfaceGravel.jpg"),
            ResourceKind::SurfaceSand => include_bytes!("../../assets/textures/SurfaceSand.jpg"),
            ResourceKind::SurfaceStone => include_bytes!("../../assets/textures/SurfaceStone.jpg"),
        };

        image::load_from_memory(bytes).map(|image| image).ok()
    }
}
