pub struct GUIRenderer {
    cx: egui::Context,
    inner_renderer: egui_wgpu::Renderer,
    window_size: (u32, u32),
    output: Option<(
        Vec<egui::ClippedPrimitive>,
        egui_wgpu::renderer::ScreenDescriptor,
    )>,
    text: String,
    count: i32,
}

impl GUIRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let cx = egui::Context::default();

        let inner_renderer = egui_wgpu::Renderer::new(
            device,
            config.format,
            Some(crate::renderer::camera::DEPTH_FORMAT),
            1,
        );

        let window_size = (config.width, config.height);

        Self {
            cx,
            inner_renderer,
            window_size,
            output: None,
            text: String::new(),
            count: 0,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, window_size: (u32, u32)) {
        self.window_size = window_size;
    }

    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        self.cx.begin_frame(egui::RawInput::default());

        // NOTE: construct gui
        egui::Window::new("GUI Example").show(&self.cx, |ui| {
            ui.label("Hello, world!");

            ui.text_edit_singleline(&mut self.text);

            let e = ui.button("Click me!");
            if e.clicked() {
                self.count += 1;
            }

            ui.label(format!("text: {}, count: {}", self.text, self.count));
        });

        let output = self.cx.end_frame();
        let paint_jobs = self.cx.tessellate(output.shapes, output.pixels_per_point);

        let (width, height) = self.window_size;
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: output.pixels_per_point,
        };

        for (id, image_delta) in &output.textures_delta.set {
            self.inner_renderer
                .update_texture(device, queue, *id, image_delta);
        }

        for id in &output.textures_delta.free {
            self.inner_renderer.free_texture(id);
        }

        self.inner_renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);

        self.output = Some((paint_jobs, screen_descriptor));
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some((paint_jobs, screen_descriptor)) = &self.output {
            self.inner_renderer
                .render(render_pass, paint_jobs, screen_descriptor);
        }
    }
}
