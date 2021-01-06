use std::collections::HashMap;

use amethyst::{assets::Loader, core::{transform::Transform}, ecs::Entity, input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, prelude::*, renderer::Camera, shred::{Dispatcher, DispatcherBuilder}, shrev::EventChannel, ui::{Anchor, FontHandle, LineMode, TtfFormat, UiImage, UiText, UiTransform}, window::ScreenDimensions};

use log::info;

use crate::{data::{ButtonAction, ButtonMap, monster::CurrMax}, systems::{combat::{CombatEvent, SpellTemplate}, eventlog::EventLogSystem}};
use crate::systems::combat::CombatEvaluationSystemDesc;
use crate::systems::eventlog::EventLogSystemDesc;
use crate::data::monster::*;

#[derive(Default)]
pub struct CombatState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for CombatState<'a, 'b> {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_camera(world, &dimensions);
 
        println!("inserting combat channel");
        let combat_channel = EventChannel::<CombatEvent>::new();
        world.insert(combat_channel);

        self.dispatcher = Some(create_state_dispatcher(world));

        let texts = vec![
        ("Claw".to_string(), ButtonAction::CastSpell{ spell_template: SpellTemplate{ damage: 10 } }), 
        ("Stab".into(),      ButtonAction::CastSpell{ spell_template: SpellTemplate{ damage: 12 } }), 
        ("".into(),          ButtonAction::Nothing), 
        ("".into(),          ButtonAction::Nothing), 
        ("".into(),          ButtonAction::Nothing), 
        ("Inventory".into(), ButtonAction::Nothing), 
        ("Escape".into(),    ButtonAction::Nothing), 
        ("End Turn".into(),  ButtonAction::Nothing)];

        create_buttons(world, 2, 4, texts);

        create_event_log(world);

        create_monster(world);

        world.insert(TurnOrder::new(vec![UnitReference::Player,
                                              UnitReference::Monster{ id: MonsterIdentifier::new(1) }]));

        world.insert(Player{ health: CurrMax{ current: 200, max: 200 }});
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

fn create_monster(world: &mut World) {
    world.create_entity()
        .with(Monster{
            id: MonsterIdentifier::new(1),
            attack: 4,
            health: CurrMax{current: 50, max: 50}})
        .build();
}

fn create_state_dispatcher<'a, 'b>(world: &mut World) -> Dispatcher<'a, 'b> {
    let mut builder = DispatcherBuilder::new();

    builder.add(
        CombatEvaluationSystemDesc::default().build(world),
        "CombatEvaluationSystem",
        &[],
    );

    builder.add(
        EventLogSystemDesc::default().build(world),
        "EventLogSystem",
        &[]
    );

    builder.build()
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}


fn create_buttons(world: &mut World, rows: u32, cols: u32, texts: Vec<(String, ButtonAction)>) {
    let width: u32 = 90;
    let height: u32 = 20;
    let start_x: u32 = 10 + width/2;
    let start_y: u32 = 10 + height/2;
    let x_offset: u32 = 100;
    let y_offset: u32 = 30;

    let mut button_map: HashMap<Entity, ButtonAction> = HashMap::new();

    // TODO, this prints the buttons in the reverse order
    for y in 0..rows {
        for x in 0..cols {
            let text = texts.get((x+y*cols) as usize).unwrap();
            button_map.insert(
                create_button(world, start_x+x*x_offset, start_y+y*y_offset, width, height, &text.0),
                text.1.clone()
            );
        }
    }
    
    world.insert(ButtonMap{ map: button_map });
}

fn create_button(world: &mut World, x: u32, y: u32, w: u32, h: u32, text: &String) -> Entity {
    let minus_x = -(x as i32);
    let ui_transform = UiTransform::new(
        String::from("simple_button".to_string() + &u32::to_string(&x) + &u32::to_string(&y)), // id
        Anchor::BottomRight,                // anchor
        Anchor::Middle,                // pivot
        minus_x as f32,                          // x
        y as f32,                          // y
        0f32,                          // z
        w as f32,                        // width
        h as f32,                         // height
    );

    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let ui_text = UiText::new(
        font,                          // font
        text.clone(), // text
        [0.0, 0.0, 0.0, 1.0],          // color
        25f32,                         // font_size
        LineMode::Single,              // line mode
        Anchor::Middle,                // alignment
    );

    /* Building the entity */
    world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(amethyst::ui::Interactable)
        .build()
}

fn create_event_log(world: &mut World)
{
    let margin = 5.;

    let background_transform = UiTransform::new(
        "".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.0,
        -10.,
        0.,
        500.,
        250.,
    );

    let mut textbox_transform = background_transform.clone();
    textbox_transform.local_x += margin;
    textbox_transform.local_y -= margin;
    textbox_transform.local_z += 1.0;

    // Blue background for the log
    world
        .create_entity()
        .with(UiImage::SolidColor([0.1, 0.1, 0.3, 1.0]))
        .with(background_transform)
        .build();

    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let event_log = world
        .create_entity()
        .with(textbox_transform)
        .with(UiText::new(
            font,
            "You wake up in a strange place, it's time to fight".to_string(),
            [1., 1., 1., 1.],
            16.,
            LineMode::Wrap,
            Anchor::TopLeft,
        ))
        .build();
    
    world.insert(EventLogText{content: event_log})
}

#[derive(Default)]
struct EventLog {
    lines: Vec<String>,
}

pub struct EventLogText {
    pub content: Entity,
}