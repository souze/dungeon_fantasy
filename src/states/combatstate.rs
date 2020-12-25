use amethyst::{assets::Loader, core::{transform::Transform}, ecs::Entity, input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, prelude::*, renderer::Camera, ui::{Anchor, FontHandle, LineMode, TtfFormat, UiButton, UiImage, UiText, UiTransform}, window::ScreenDimensions};

use log::info;

/// A dummy game state that shows 3 sprites.
pub struct CombatState;

impl SimpleState for CombatState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        create_ui_example(world);

        let texts = vec!["Claw".to_string(), "Stab".into(), "Fireball".into(), "Chicken".into(), 
                                           "Suicide".into(), "Inventory".into(), "".into(), "Escape".into()];
        let buttons = create_buttons(world, 2, 4, texts);
        println!("{:?}", buttons)
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

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}


fn create_buttons(world: &mut World, rows: u32, cols: u32, texts: Vec<String>) -> Vec<Entity> {
    let width: u32 = 90;
    let height: u32 = 20;
    let start_x: u32 = 10 + width/2;
    let start_y: u32 = 10 + height/2;
    let x_offset: u32 = 100;
    let y_offset: u32 = 30;

    let mut buttons: Vec<Entity> = Vec::new();

    for y in 0..rows {
        for x in 0..cols {
            // TODO, how do i get the x + y*rows?
            let text: &String = texts.get((x+y*cols) as usize).unwrap();
            buttons.push(create_button(world, start_x+x*x_offset, start_y+y*y_offset,
                width, height, text));
        }
    }
    buttons
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

/// Creates a simple UI background and a UI text label
/// This is the pure code only way to create UI with amethyst.
pub fn create_ui_example(world: &mut World) {
    // this creates the simple gray background UI element.
    world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            30.0,
            -30.,
            0.,
            250.,
            50.,
        ))
        .build();

    // This simply loads a font from the asset folder and puts it in the world as a resource,
    // we also get a ref to the font that we then can pass to the text label we crate later.
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // This creates the actual label and places it on the screen.
    // Take note of the z position given, this ensures the label gets rendered above the background UI element.
    world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            40.0,
            -40.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "Hello, Amethyst UI!".to_string(),
            [1., 1., 1., 1.],
            30.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();
}
