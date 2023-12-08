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

    let gui_cx = egui::Context::default();
    let mut gui = egui_winit::State::new(gui_cx.viewport_id(), &window, None, None);

    event_loop.run(move |event, _, control_flow| {
        let _ = input.update(&event);

        match event {
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let tick = std::mem::replace(&mut instant, std::time::Instant::now()).elapsed();

                let gui_input = gui.take_egui_input(&window);
                gui_cx.begin_frame(gui_input);

                let window_size = (window.inner_size().width, window.inner_size().height);
                let cx = game_loop::Context {
                    assets: &assets,
                    input: &input,
                    tick: &tick,
                    window_size: &window_size,
                    gui_cx: &gui_cx,
                };
                game_loop.update(&cx);
                let extract = game_loop.extract(&cx);

                let gui_output = gui_cx.end_frame();
                renderer.render(&assets, &extract, &gui_cx, gui_output);
            }
            Event::WindowEvent { event, .. } => {
                let _ = gui.on_window_event(&gui_cx, &event);

                match event {
                    WindowEvent::Resized(new_inner_size) => {
                        renderer.resize(new_inner_size);
                    }
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    });
}
