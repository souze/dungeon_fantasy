use std::string;

use egui::{Color32, Label, TextStyle, Ui};

use crate::game::world::World;

pub struct UiState {
    pub eventlog_entries: Vec<String>,
}

pub fn draw_ui(ctx: &mut egui::CtxRef, _w: &World, state: &mut UiState) {
    let mut show_egui_demo_windows: bool = true;

    egui::Window::new("Debug").show(ctx, |ui| {
        ui.add(egui::Label::new("Egui on Macroquad").text_style(egui::TextStyle::Heading));
        ui.separator();
        ui.checkbox(&mut show_egui_demo_windows, "Show egui demo windows");
        ui.label("Woooohoooo!");
        if ui.button("Quit").clicked {
            std::process::exit(0);
        }
    });

    egui::Window::new("Event Log").show(ctx, |ui| {
        populate_event_log(ui);
    });

    egui::Area::new("Whatever")
        .fixed_pos(egui::pos2(32.0, 32.0))
        .show(ctx, |ui| {
            for spell in &["Stab", "Claw", "Fireball", "Escape"] {
                if ui.button(spell.to_string()).clicked {
                    println!("clicked the spell {}", spell)
                }
            }
        });
}

fn populate_event_log(ui: &mut Ui) {
        "You cast Astro Blast on Scarecrow, it hits for 120 physical damage";
        let spell_color = Color32::from_rgb(100, 150, 0);
        let you_color = Color32::from_rgb(0, 50, 200);
        let enemy_color = Color32::from_rgb(150, 50, 50);
        let damage_color = Color32::from_rgb(200, 20, 20);
        ui.horizontal_wrapped_for_text(TextStyle::Body, |ui| {
            ui.colored_label(you_color, "You");
            ui.label("cast");
            ui.colored_label(spell_color, "Astro Blast");
            ui.label("on");
            ui.colored_label(enemy_color, "Scarecrow");
            ui.label("it hits for");
            ui.colored_label(damage_color, "120 physical");
            ui.label("damage");
        });
}