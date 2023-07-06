mod camera;
mod generation;
mod interaction;
mod iunit;
mod unit;

pub use camera::CameraService;
pub use generation::GenerationService;
pub use interaction::InteractionService;
pub use iunit::IUnitService;
pub use unit::UnitService;

use glam::*;

pub struct ReadBack {
    pub screen_to_world: Option<Mat4>,
}

pub struct Service {
    pub camera_service: CameraService,
    pub iunit_service: IUnitService,
    pub unit_service: UnitService,
    pub generation_service: GenerationService,
    pub interaction_service: InteractionService,
    time_instance: std::time::Instant,
}

impl Service {
    pub fn new() -> Self {
        Self {
            camera_service: CameraService::default(),
            iunit_service: IUnitService::default(),
            unit_service: UnitService::default(),
            generation_service: GenerationService::default(),
            interaction_service: InteractionService::default(),
            time_instance: std::time::Instant::now(),
        }
    }

    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: Option<&ReadBack>,
    ) {
        let elapsed = self.time_instance.elapsed();

        if self.camera_service.get_camera().is_none() {
            self.camera_service.spawn_camera();
        }

        self.camera_service.update(input, elapsed);

        if let Some(read_back) = read_back {
            self.interaction_service
                .update(input, read_back, &mut self.iunit_service);
        }

        if let Some(camera) = self.camera_service.get_camera() {
            let mut bounds = camera.view_area();

            const MARGIN: f32 = 2.0;
            bounds.min -= Vec3A::splat(MARGIN);
            bounds.max += Vec3A::splat(MARGIN);

            self.generation_service
                .generate(bounds.into(), &mut self.iunit_service);
        }

        self.time_instance = std::time::Instant::now();
    }
}
