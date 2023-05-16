mod models;
mod services;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let window = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let renderer = pollster::block_on(Renderer::new_async(&window));

    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            renderer.draw();
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => {}
        },
        _ => {}
    })
}

struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}

impl Renderer {
    pub async fn new_async(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let inner_size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, inner_size.width, inner_size.height)
            .unwrap();
        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
        }
    }

    pub fn draw(&self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations::default(),
            })],
            depth_stencil_attachment: None,
        });
        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
