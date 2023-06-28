mod camera;
mod generation;
mod iunit;
mod unit;

pub use camera::CameraService;
pub use generation::GenerationService;
pub use iunit::IUnitService;
pub use unit::UnitService;

pub struct Service {
    pub camera_service: CameraService,
    pub iunit_service: IUnitService,
    pub unit_service: UnitService,
    pub generation_service: GenerationService,
}

impl Service {
    pub fn new() -> Self {
        Self {
            camera_service: CameraService::default(),
            iunit_service: IUnitService::default(),
            unit_service: UnitService::default(),
            generation_service: GenerationService::default(),
        }
    }

    pub fn update(&mut self) {
        if self.camera_service.get_camera().is_none() {
            self.camera_service.spawn_camera();
        }

        if let Some(camera) = self.camera_service.get_camera() {
            let bounds = camera.view_area;

            self.generation_service
                .generate(bounds.into(), &mut self.iunit_service);
        }
    }
}
