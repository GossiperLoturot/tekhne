#[derive(Default)]
pub enum UIService {
    #[default]
    None,
    Inventory,
}

impl UIService {
    pub fn update(&mut self, input: &winit_input_helper::WinitInputHelper) {
        match self {
            UIService::None => {
                if input.key_pressed(winit::event::VirtualKeyCode::E) {
                    *self = UIService::Inventory;
                }
            }
            UIService::Inventory => {
                if input.key_pressed(winit::event::VirtualKeyCode::E)
                    | input.key_pressed(winit::event::VirtualKeyCode::Escape)
                {
                    *self = UIService::None;
                }
            }
        }
    }
}
