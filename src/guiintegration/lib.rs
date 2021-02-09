use crate::guiintegration::input;
use crate::guiintegration::painter;

use macroquad::{miniquad, prelude::KeyCode};

use super::egui_key_from_mq_key;

pub trait UiDrawer {
    fn draw_ui(&mut self, egui_ctx: &mut egui::CtxRef);
}

/// egui bindings for miniquad
pub struct EguiMq<'a> {
    egui_ctx: egui::CtxRef,
    egui_input: egui::RawInput,
    mq_ctx: &'a mut miniquad::Context,
    painter: painter::Painter,
}

impl EguiMq<'_> {
    pub fn new() -> Self {
        let mut mq_ctx = {
            let macroquad::prelude::InternalGlContext {
                quad_context: ctx, ..
            } = unsafe { macroquad::prelude::get_internal_gl() };

            ctx
        };

        Self {
            egui_ctx: egui::CtxRef::default(),
            painter: painter::Painter::new(&mut mq_ctx),
            egui_input: Default::default(),
            mq_ctx,
        }
    }

    pub fn update<Drawer>(&mut self, ui_drawer: &mut Drawer) where
            Drawer: UiDrawer {

        self.handle_inputs();

        self.mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        self.mq_ctx.begin_default_pass(miniquad::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();
        self.begin_frame();

        ui_drawer.draw_ui(&mut self.egui_ctx);

        self.end_frame();

        self.mq_ctx.commit_frame();
    }

    fn handle_inputs(&mut self) {
        self.handle_mouse_inputs();

        self.handle_keyboard_inputs();
    }

    /// Call this at the start of each `draw` call.
    pub fn begin_frame(&mut self) {
        input::on_frame_start(&mut self.egui_input, self.mq_ctx);
        self.egui_ctx.begin_frame(self.egui_input.take());
    }

    /// Call this at the end of each `draw` call.
    pub fn end_frame(&mut self) {
        let (output, shapes) = self.egui_ctx.end_frame();
        let paint_jobs = self.egui_ctx.tessellate(shapes);

        let egui::Output {
            cursor_icon: _, // https://github.com/not-fl3/miniquad/issues/171
            open_url: _, // We don't handle urls
            copied_text: _, // we don't handle copy-paste (yet)
            needs_repaint: _, // miniquad always runs at full framerate
        } = output;

        self.painter
            .paint(self.mq_ctx, paint_jobs, &self.egui_ctx.texture());
    }

    fn handle_mouse_inputs(&mut self) {
        use macroquad::input as inp;

        let dpi_scale = self.mq_ctx.dpi_scale();

        let (x, y) = inp::mouse_position();
        self.egui_input.mouse_pos = Some(egui::pos2(
            x as f32 / dpi_scale,
            y as f32 / dpi_scale,
        ));

        if inp::is_mouse_button_down(inp::MouseButton::Left) {
            self.egui_input.mouse_down = true;
        } else if inp::is_mouse_button_released(inp::MouseButton::Left) {
            self.egui_input.mouse_down = false;
        }

        {
            let (scroll_delta_x, scroll_delta_y) = inp::mouse_wheel();
            self.egui_input.scroll_delta += egui::vec2(scroll_delta_x, scroll_delta_y) * dpi_scale;
        }
    }

    fn handle_keyboard_inputs(&mut self) {
        use macroquad::input as inp;

        let ctrl_pressed = inp::is_key_down(KeyCode::LeftControl) ||
                                inp::is_key_down(KeyCode::RightControl);
        self.egui_input.modifiers.ctrl = ctrl_pressed;
        self.egui_input.modifiers.command = ctrl_pressed;
        self.egui_input.modifiers.shift = inp::is_key_down(KeyCode::LeftShift) ||
                                          inp::is_key_down(KeyCode::RightShift);
        self.egui_input.modifiers.alt = inp::is_key_down(KeyCode::LeftAlt) ||
                                        inp::is_key_down(KeyCode::RightShift);

        while let Some(c) = inp::get_char_pressed() {
            self.char_event(c);
        }

        while let Some(mq_key) = inp::get_last_key_pressed() {
            if let Some(egui_key) = egui_key_from_mq_key(mq_key) {
                self.egui_input.events.push(egui::Event::Key {
                    key: egui_key, 
                    modifiers: self.egui_input.modifiers,
                    pressed: true,
                })
            }
        }
    }

    fn char_event(&mut self, chr: char) {
        if input::is_printable_char(chr)
            && !self.egui_input.modifiers.ctrl
            && !self.egui_input.modifiers.mac_cmd
        {
            self.egui_input
                .events
                .push(egui::Event::Text(chr.to_string()));
        }
    }
}