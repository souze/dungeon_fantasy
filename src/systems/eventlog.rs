// use amethyst::{shred::{Read, System}, shrev::{EventChannel, ReaderId}};
// use amethyst::{derive::*, shred::{Read, System}, shrev::ReaderId};
use amethyst::{ecs::WriteStorage, prelude::*, shred::{Read, ReadExpect, System, SystemData, Write}, shrev::{EventChannel, ReaderId}};
use amethyst::derive::*;

use super::combat::CombatEvent;
use crate::states::EventLogText;

#[derive(SystemDesc)]
#[system_desc(name(EventLogSystemDesc))]
pub struct EventLogSystem {
    #[system_desc(event_channel_reader)]
    combat_reader_id: ReaderId<CombatEvent>,
}

impl EventLogSystem {
    pub fn new(combat_reader_id: ReaderId<CombatEvent>) -> Self {
        Self{combat_reader_id}
    }
}

impl<'s> System<'s> for EventLogSystem {
    type SystemData = (Read<'s, EventChannel<CombatEvent>>,
                    ReadExpect<'s, EventLogText>,
                    WriteStorage<'s, amethyst::ui::UiText>);

    fn run(&mut self, (events, event_log_text, mut ui_texts): Self::SystemData) {
        for event in events.read(&mut self.combat_reader_id) {
            println!("Event Log Received combat event: {:?}", event);
            let mut text = ui_texts.get_mut(event_log_text.content).unwrap();
            text.text = format!("{}\n{:?}", text.text, event);
        }
    }
}