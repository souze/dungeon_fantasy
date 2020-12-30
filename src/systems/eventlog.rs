// use amethyst::{shred::{Read, System}, shrev::{EventChannel, ReaderId}};
// use amethyst::{derive::*, shred::{Read, System}, shrev::ReaderId};
use amethyst::{prelude::*, shred::{Read, System, SystemData}, shrev::{EventChannel, ReaderId}};
use amethyst::derive::*;

use super::combat::CombatEvent;

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
    type SystemData = Read<'s, EventChannel<CombatEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        for event in events.read(&mut self.combat_reader_id) {
            println!("Event Log Received combat event: {:?}", event)
        }
    }
}