use crate::renderer;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct GuiResourceHandle {
    paint_jobs: Vec<egui::ClippedPrimitive>,
    screen_descriptor: egui_wgpu::renderer::ScreenDescriptor,
}

pub struct GuiRenderer {
    inner_renderer: egui_wgpu::Renderer,
}

impl GuiRenderer {
    pub fn new(render_state: &renderer::RenderState) -> Self {
        let device = &render_state.device;

        let output_color_format = render_state.config.format;
        let output_depth_format = Some(DEPTH_FORMAT);
        let msaa_samples = 1;
        let inner_renderer = egui_wgpu::Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
        );

        Self { inner_renderer }
    }

    pub fn upload(
        &mut self,
        render_state: &mut renderer::RenderState,
        encoder: &mut wgpu::CommandEncoder,
        gui_cx: &egui::Context,
        gui_output: egui::FullOutput,
    ) -> GuiResourceHandle {
        let device = &render_state.device;
        let queue = &render_state.queue;
        let config = &render_state.config;

        let paint_jobs = gui_cx.tessellate(gui_output.shapes, gui_cx.pixels_per_point());
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [config.width, config.height],
            pixels_per_point: gui_cx.pixels_per_point(),
        };
        self.inner_renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);

        for (id, image_delta) in gui_output.textures_delta.set {
            self.inner_renderer
                .update_texture(device, queue, id, &image_delta);
        }

        for id in gui_output.textures_delta.free {
            self.inner_renderer.free_texture(&id);
        }

        GuiResourceHandle {
            paint_jobs,
            screen_descriptor,
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        handle: &'a GuiResourceHandle,
    ) {
        let paint_jobs = &handle.paint_jobs;
        let screen_descriptor = &handle.screen_descriptor;
        self.inner_renderer
            .render(render_pass, paint_jobs, screen_descriptor);
    }
}
