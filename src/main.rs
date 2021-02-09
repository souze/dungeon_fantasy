use guiintegration::{EguiMq, UiDrawer};
use macroquad::prelude::{next_frame};
use ui::UiState;

mod guiintegration;
mod ui;
mod game;

struct UiDrawerCurringWorld<'a> {
    w: &'a game::World,
    ui_state: &'a mut ui::draw::UiState,
}

impl<'a> UiDrawer for UiDrawerCurringWorld<'a> {
    fn draw_ui(&mut self, egui_ctx: &mut egui::CtxRef) {
        ui::draw::draw_ui(egui_ctx, &self.w, &mut self.ui_state);
    }
}

#[macroquad::main("dungeon_fantasy")]
async fn main() {
    let mut egui_mq = EguiMq::new();
    let mut world = game::World{};
    let mut ui_state = ui::draw::UiState{eventlog_entries: vec!["You take 10 physacal damage from bleed".to_string()]};

    loop {
        game::gameloop::gameloop(&mut world);

        egui_mq.update(&mut UiDrawerCurringWorld{w: &world, ui_state: &mut ui_state});

        next_frame().await;
    }
}
