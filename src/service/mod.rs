mod generation;
mod iunit;
mod player;
mod unit;

pub use generation::GenerationService;
pub use iunit::IUnitService;
pub use player::PlayerService;
pub use unit::UnitService;

pub struct Service {
    pub player_service: PlayerService,
    pub iunit_service: IUnitService,
    pub unit_service: UnitService,
    pub generation_service: GenerationService,
}

impl Service {
    pub fn new() -> Self {
        Self {
            player_service: PlayerService::default(),
            iunit_service: IUnitService::default(),
            unit_service: UnitService::default(),
            generation_service: GenerationService::default(),
        }
    }

    pub fn update(&mut self) {
        if self.player_service.get_player().is_none() {
            self.player_service.spawn_player();
        }

        if let Some(player) = self.player_service.get_player() {
            let bounds = player.view_area;

            self.generation_service
                .generate(bounds.into(), &mut self.iunit_service);
        }
    }
}
