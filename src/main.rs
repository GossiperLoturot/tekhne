use winit::event::Event;
use winit::event::WindowEvent;

mod assets;
mod game_loop;
mod renderer;

fn main() {
    let assets = assets::Assets::new("assets/assets.json");

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut game_loop = game_loop::GameLoop::new();
    let mut renderer = pollster::block_on(renderer::Renderer::new_async(&assets, &window));
    let mut input = winit_input_helper::WinitInputHelper::new();
    let mut instant = std::time::Instant::now();

    let gui_cx = egui::Context::default();
    let viewport_id = gui_cx.viewport_id();
    let mut gui = egui_winit::State::new(gui_cx, viewport_id, &window, None, None);

    event_loop
        .run(move |event, control_flow| {
            let _ = input.update(&event);

            match event {
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::WindowEvent { event, .. } => {
                    let _ = gui.on_window_event(&window, &event);

                    match event {
                        WindowEvent::RedrawRequested => {
                            let tick = std::mem::replace(&mut instant, std::time::Instant::now())
                                .elapsed();

                            let gui_input = gui.take_egui_input(&window);
                            gui.egui_ctx().begin_frame(gui_input);

                            let window_size =
                                (window.inner_size().width, window.inner_size().height);
                            let cx = game_loop::Context {
                                assets: &assets,
                                input: &input,
                                tick: &tick,
                                window_size: &window_size,
                                gui_cx: gui.egui_ctx(),
                            };
                            game_loop.update(&cx);
                            let extract = game_loop.extract(&cx);

                            let gui_output = gui.egui_ctx().end_frame();
                            renderer.render(&assets, &extract, &gui.egui_ctx(), gui_output);
                        }
                        WindowEvent::Resized(new_inner_size) => {
                            renderer.resize(new_inner_size);
                        }
                        WindowEvent::CloseRequested => {
                            control_flow.exit();
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        })
        .unwrap();
}
