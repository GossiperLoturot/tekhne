mod assets;
mod game_loop;
mod renderer;

fn main() {
    let assets = assets::Assets::new("assets/assets.json");

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let window = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut game_loop = game_loop::GameLoop::new();
    let mut renderer = pollster::block_on(renderer::Renderer::new_async(&assets, &window));
    let mut input = winit_input_helper::WinitInputHelper::new();
    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop
        .run(move |event, control_flow| {
            input.update(&event);

            match event {
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::WindowEvent { window_id, event } if window_id == window.id() => {
                    match event {
                        WindowEvent::RedrawRequested => {
                            game_loop.update(&assets, &input);
                            renderer.draw(&assets, &game_loop);
                        }
                        WindowEvent::CloseRequested => {
                            control_flow.exit();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
