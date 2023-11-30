use winit::event::Event;
use winit::event::WindowEvent;

mod assets;
mod game_loop;
mod renderer;

fn main() {
    let assets = assets::Assets::new("assets/assets.json");

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut game_loop = game_loop::GameLoop::new();
    let mut renderer = pollster::block_on(renderer::Renderer::new_async(&assets, &window));
    let mut input = winit_input_helper::WinitInputHelper::new();
    let mut instant = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        input.update(&event);

        match event {
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let tick = std::mem::replace(&mut instant, std::time::Instant::now()).elapsed();

                let window_size = (window.inner_size().width, window.inner_size().height);

                let cx = game_loop::Context {
                    assets: &assets,
                    input: &input,
                    tick: &tick,
                    window_size: &window_size,
                };
                game_loop.update(&cx);
                let extract = game_loop.extract(&cx);

                renderer.draw(&assets, &extract);
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(inner_size) => {
                    let window_size = (inner_size.width, inner_size.height);
                    renderer.resize(window_size);
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => (),
            },
            _ => (),
        }
    });
}
