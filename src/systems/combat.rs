use amethyst::{ecs::{ReadStorage, WriteStorage}, prelude::*, shred::{Read, ReadExpect, System, SystemData, Write, WriteExpect}, shrev::{EventChannel, ReaderId}, ui::{UiEvent, UiEventType}};
use amethyst::derive::*;
use amethyst::ecs::Join;

use crate::data::{ButtonAction, ButtonMap, Player, TurnOrder, monster::{Monster, MonsterIdentifier}};
use crate::data::monster::UnitReference;

#[derive(Debug)]
pub struct CombatEvent {
    source: String,
    target: UnitReference,
    spell_template:  SpellTemplate,

    results: Vec<CombatResult>,
}

impl CombatEvent {
    fn new(source: String, target: UnitReference, spell_template: SpellTemplate) -> Self {
        Self{source, target, spell_template, results: Vec::new()}
    }

    fn add_result(&mut self, result: CombatResult) {
        self.results.push(result);
    }
}

#[derive(Debug)]
enum CombatResult {
    TakeDamage(u32),
    Death,
}

#[derive(Debug, Clone)]
pub struct SpellTemplate {
    pub damage: u32,
}

#[derive(SystemDesc)]
#[system_desc(name(CombatEvaluationSystemDesc))]
pub struct CombatEvaluationSystem {
    #[system_desc(event_channel_reader)]
    combat_reader_id: ReaderId<CombatEvent>,
    #[system_desc(event_channel_reader)]
    ui_reader_id: ReaderId<amethyst::ui::UiEvent>,
}

impl CombatEvaluationSystem {
    pub fn new(combat_reader_id: ReaderId<CombatEvent>, ui_reader_id: ReaderId<amethyst::ui::UiEvent>) -> Self {
        Self{combat_reader_id, ui_reader_id}
    }
}

impl<'s> System<'s> for CombatEvaluationSystem {
    type SystemData = (
                    WriteExpect<'s, TurnOrder>,
                    ReadExpect<'s, ButtonMap>,
                    Read<'s, EventChannel<amethyst::ui::UiEvent>>,
                    Write<'s, EventChannel<CombatEvent>>,
                    WriteStorage<'s, Monster>,
                    WriteExpect<'s, Player>,
                    );

    fn run(&mut self, (mut turn_order, 
                        button_map,
                        ui_events, 
                        mut combat_events, 
                        mut monsters,
                        mut player): 
                        Self::SystemData) {
        let mut combat_event: Option<CombatEvent> = None;

        if turn_order.current() == UnitReference::Player {
            for event in ui_events.read(&mut self.ui_reader_id) {
                if event.event_type == UiEventType::ClickStart {
                    match button_map.map.get(&event.target) {
                        Some(ButtonAction::CastSpell { spell_template }) => {
                            println!("Casting {:?}", spell_template);
                            combat_event = Some(CombatEvent::new(
                                "You".to_owned(),
                                UnitReference::Monster{ id: MonsterIdentifier::new(1) },
                                spell_template.clone()))
                            },
                        Some(ButtonAction::Nothing) => println!("Pressed a nothing button"),
                        None => println!("Button not mapped"),
                    }
                }
            }
        } else {
            combat_event = Some(CombatEvent::new(
                "Gnoll".to_owned(),
                UnitReference::Player,
                SpellTemplate{damage: 12}));
        }

        let mut event = match combat_event {
            None => return,
            Some(ev) => ev
        };
        
        println!("Received combat event: {:?}", event);
        
        let spell_target = event.target.clone();
        match spell_target {
            UnitReference::Player => {
                handle_player_target(&mut player, &mut event);
            }
            UnitReference::Monster { id } => {
                for mut potential_monster in (&mut monsters).join() {
                    println!("Monster eval: {:?}", potential_monster);
                    if potential_monster.id == id {
                        handle_monster_target(&mut potential_monster, &mut event);
                    }
                }
            }
        }
        
        combat_events.single_write(event);

        turn_order.advance();
    }
}

fn handle_player_target(player: &mut Player, event: &mut CombatEvent) -> () {
    let remaining_hp: i32 = player.health.current - event.spell_template.damage as i32;

    if remaining_hp > 0 {
        event.add_result(CombatResult::TakeDamage(event.spell_template.damage));
        player.health.current -= event.spell_template.damage as i32;
    } else {
        event.add_result(CombatResult::TakeDamage(player.health.current as u32));
        event.add_result(CombatResult::Death);
        player.health.current = 0;
    }
}

fn handle_monster_target(monster: &mut Monster, event: &mut CombatEvent) {
    // This monster is the target of the spell in CombatEvent, what happens? Take damage etc.

    let monster_hp: i32 = monster.health.current - event.spell_template.damage as i32;
    if monster_hp > 0 {
        event.add_result(CombatResult::TakeDamage(event.spell_template.damage));
        monster.health.current -= event.spell_template.damage as i32;
    } else {
        event.add_result(CombatResult::TakeDamage(monster.health.current as u32));
        event.add_result(CombatResult::Death);
        monster.health.current = 0;
    }
}