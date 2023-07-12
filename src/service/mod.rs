pub use camera::CameraService;
pub use generation::GenerationService;
pub use interaction::InteractionService;
pub use iunit::IUnitService;
pub use player::PlayerService;
pub use unit::UnitService;

use glam::*;

mod camera;
mod generation;
mod interaction;
mod iunit;
mod player;
mod unit;

pub struct ReadBack {
    pub screen_to_world: Option<Mat4>,
}

pub struct Service {
    pub camera: CameraService,
    pub iunit: IUnitService,
    pub unit: UnitService,
    pub generation: GenerationService,
    pub interaction: InteractionService,
    pub player: PlayerService,
    time_instant: std::time::Instant,
}

impl Service {
    pub fn new() -> Self {
        Self {
            camera: CameraService::default(),
            iunit: IUnitService::default(),
            unit: UnitService::default(),
            generation: GenerationService::default(),
            interaction: InteractionService::default(),
            player: PlayerService::default(),
            time_instant: std::time::Instant::now(),
        }
    }

    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: Option<&ReadBack>,
    ) {
        let elapsed = self.time_instant.elapsed();

        if self.player.get_player().is_none() {
            self.player.spawn_player();
        }

        if self.camera.get_camera().is_none() {
            self.camera.spawn_camera();
        }

        self.player.update(input, elapsed);
        self.camera.update(&self.player);

        if let Some(read_back) = read_back {
            self.interaction.update(input, read_back, &mut self.iunit);
        }

        if let Some(camera) = self.camera.get_camera() {
            let mut bounds = camera.view_area();

            const MARGIN: f32 = 2.0;
            bounds.min -= Vec3A::splat(MARGIN);
            bounds.max += Vec3A::splat(MARGIN);

            self.generation.generate(bounds.into(), &mut self.iunit);
        }

        self.time_instant = std::time::Instant::now();
    }
}
