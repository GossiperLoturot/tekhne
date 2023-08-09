pub use camera::CameraService;
pub use generation::GenerationService;
pub use interaction::InteractionService;
pub use iunit::IUnitService;
pub use player::PlayerService;
pub use ui::UIService;
pub use unit::UnitService;

use glam::*;

mod camera;
mod generation;
mod interaction;
mod iunit;
mod player;
mod ui;
mod unit;

pub struct ReadBack {
    pub screen_to_world_matrix: Option<Mat4>,
    pub screen_to_ui_matrix: Option<Mat4>,
}

pub struct Service {
    pub camera: CameraService,
    pub iunit: IUnitService,
    pub unit: UnitService,
    pub generation: GenerationService,
    pub interaction: InteractionService,
    pub player: PlayerService,
    pub ui: UIService,
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
            ui: UIService::default(),
            time_instant: std::time::Instant::now(),
        }
    }

    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: Option<&ReadBack>,
    ) {
        let elapsed = self.time_instant.elapsed();
        self.time_instant = std::time::Instant::now();

        if self.player.get_player(&self.unit).is_none() {
            self.player.spawn_player(&mut self.unit);
        }

        if self.camera.get_camera().is_none() {
            self.camera.spawn_camera();
        }

        self.player.update(&mut self.unit, input, elapsed);
        self.camera.update(&self.unit, &self.player, input);
        self.ui.update(input);

        if let Some(read_back) = read_back {
            self.interaction
                .update(input, read_back, &mut self.iunit, &mut self.unit);
        }

        if let Some(camera) = self.camera.get_camera() {
            self.generation.generate(
                camera.view_aabb().floor().as_iaabb3(),
                &mut self.iunit,
                &mut self.unit,
            );
        }
    }
}
