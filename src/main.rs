use winit::event::Event;
use winit::event::WindowEvent;

mod aabb;
mod assets;
mod game_loop;
mod renderer;

fn main() {
    let assets = assets::Assets::new("assets/assets.json");
    let assets = std::rc::Rc::new(assets);

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    let window = std::rc::Rc::new(window);

    let mut game_loop = game_loop::GameLoop::new(assets.clone());
    let mut renderer = pollster::block_on(renderer::RenderingSystem::new_async(
        assets.clone(),
        window.clone(),
    ));
    let mut input = winit_input_helper::WinitInputHelper::new();
    let mut instant = std::time::Instant::now();

    event_loop
        .run(move |event, control_flow| {
            let _ = input.update(&event);

            match event {
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested => {
                        let tick =
                            std::mem::replace(&mut instant, std::time::Instant::now()).elapsed();
                        let window_size = (window.inner_size().width, window.inner_size().height);

                        game_loop.update(&input, &tick, window_size);

                        let extract = game_loop.extract(window_size);

                        renderer.draw(&extract);
                    }
                    WindowEvent::Resized(new_inner_size) => {
                        renderer.resize(new_inner_size);
                    }
                    WindowEvent::CloseRequested => {
                        control_flow.exit();
                    }
                    _ => (),
                },
                _ => (),
            }
        })
        .unwrap();
}
