use crate::{app::UserState, TileType};
use imgui::*;
use imgui_wgpu::RendererConfig;
use pixels::{wgpu, PixelsContext};
use std::time::Instant;
use strum::IntoEnumIterator;

pub struct Gui {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
}

impl Gui {
    pub fn new(
        window: &winit::window::Window,
        pixels: &pixels::Pixels<winit::window::Window>,
    ) -> Self {
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );

        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        let style = imgui.style_mut();
        for color in 0..style.colors.len() {
            style.colors[color] = gamma_to_linear(style.colors[color]);
        }

        let device = pixels.device();
        let queue = pixels.queue();
        let config = RendererConfig {
            texture_format: wgpu::TextureFormat::Bgra8UnormSrgb,
            ..Default::default()
        };
        let renderer = imgui_wgpu::Renderer::new(&mut imgui, &device, &queue, config);

        Self {
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
        }
    }

    pub fn prepare(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), winit::error::ExternalError> {
        let io = self.imgui.io_mut();
        let last_frame = Instant::now();
        let delta = last_frame - self.last_frame;
        io.update_delta_time(delta);
        self.last_frame = last_frame;
        self.platform.prepare_frame(io, window)
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
        user_state: &mut UserState,
    ) -> imgui_wgpu::RendererResult<()> {
        let ui = self.imgui.frame();

        let win = Window::new(im_str!("poussière"));
        win.build(&ui, || {
            ui.text("Materials");

            // material radio buttons

            for tile_type in TileType::iter() {
                let name: &'static str = tile_type.into();
                if ui.radio_button_bool(&ImString::new(name), tile_type == user_state.current_tile)
                {
                    user_state.current_tile = tile_type;
                };
            }

            ui.new_line();

            // brush size selector

            ui.text("Brush size");
            if ui.small_button(im_str!("-")) && user_state.brush_size > 1 {
                user_state.brush_size -= 1;
            }
            ui.same_line_with_spacing(32f32, 0f32);
            ui.text(format!("{}", user_state.brush_size));
            ui.same_line_with_spacing(50f32, 0f32);
            if ui.small_button(im_str!("+")) {
                user_state.brush_size += 1;
            }
            ui.new_line();

            ui.text("World");

            let text: &ImStr = if user_state.running {
                &im_str!("Stop")
            } else {
                &im_str!("Start")
            };

            if ui.small_button(text) {
                user_state.running = !user_state.running;
            }

            ui.new_line();

            if ui.small_button(&im_str!("Clear World")) {
                user_state.edit_action_flag = Some(crate::app::EditAction::Clear);
            }

            ui.new_line();

            // undo handling

            let disabled_undo = if user_state.action_stack_size == 0 {
                (ui.push_style_var(StyleVar::Alpha(0.1)), true)
            } else {
                (ui.push_style_var(StyleVar::Alpha(1.0)), false)
            };

            if ui.small_button(&im_str!("Undo")) && !disabled_undo.1 {
                user_state.edit_action_flag = Some(crate::app::EditAction::Undo);
            }

            disabled_undo.0.pop(&ui);

            ui.new_line();
        });

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.renderer
            .render(ui.render(), &context.queue, &context.device, &mut rpass)
    }

    pub fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<()>,
    ) -> bool {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);

        self.imgui.io().want_capture_mouse
    }
}

fn gamma_to_linear(color: [f32; 4]) -> [f32; 4] {
    const GAMMA: f32 = 2.2;

    let x = color[0].powf(GAMMA);
    let y = color[1].powf(GAMMA);
    let z = color[2].powf(GAMMA);
    let w = 1.0 - (1.0 - color[3]).powf(GAMMA);

    [x, y, z, w]
}
