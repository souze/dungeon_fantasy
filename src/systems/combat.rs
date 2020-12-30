use amethyst::{prelude::*, shred::{Read, System, Write, SystemData}, shrev::{EventChannel, ReaderId}, ui::UiEventType};
use amethyst::derive::*;

#[derive(SystemDesc)]
#[system_desc(name(PlayerTurnSystemDesc))]
pub struct PlayerTurnSystem {
    #[system_desc(event_channel_reader)]
    ui_reader_id: ReaderId<amethyst::ui::UiEvent>,
}

impl<'s> System<'s> for PlayerTurnSystem {
    type SystemData = (
        Read<'s, EventChannel<amethyst::ui::UiEvent>>,
        Write<'s, EventChannel<CombatEvent>>);

    fn run(&mut self, (ui_events, mut combat_events): Self::SystemData) {
        for event in ui_events.read(&mut self.ui_reader_id) {
            if event.event_type == UiEventType::ClickStart {
                let combat_event: CombatEvent = CombatEvent{source: "You".into(), target: "Gnoll".into(), spell_template: SpellTemplate{damage: 42}};
                println!("Putting combat event on the channel");
                combat_events.single_write(combat_event);
            }
        }
    }
}

#[derive(Debug)]
pub struct CombatEvent {
    source: String,
    target: String,
    spell_template:  SpellTemplate,
}

#[derive(Debug)]
struct SpellTemplate {
    damage: u32,
}

impl PlayerTurnSystem {
    pub fn new(ui_reader_id: amethyst::shrev::ReaderId<amethyst::ui::UiEvent>) -> Self {
        Self {
            ui_reader_id,
        }
    }
}

#[derive(SystemDesc)]
#[system_desc(name(CombatEvaluationSystemDesc))]
pub struct CombatEvaluationSystem {
    #[system_desc(event_channel_reader)]
    combat_reader_id: ReaderId<CombatEvent>,
}

impl CombatEvaluationSystem {
    pub fn new(combat_reader_id: ReaderId<CombatEvent>) -> Self {
        Self{combat_reader_id}
    }
}

impl<'s> System<'s> for CombatEvaluationSystem {
    type SystemData = Read<'s, EventChannel<CombatEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        for event in events.read(&mut self.combat_reader_id) {
            println!("Received combat event: {:?}", event)
        }
    }
}