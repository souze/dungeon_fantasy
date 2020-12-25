use amethyst::{prelude::*, shred::Write};

pub struct SimpleButtonSystem {
    reader_id: amethyst::shrev::ReaderId<amethyst::ui::UiEvent>,
}

impl<'s> amethyst::shred::System<'s> for SimpleButtonSystem {
    type SystemData = amethyst::shred::Read<'s, amethyst::shrev::EventChannel<amethyst::ui::UiEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        for event in events.read(&mut self.reader_id) {
            println!("{:?}", event);
        }
    }
}

impl SimpleButtonSystem {
    pub fn new(reader_id: amethyst::shrev::ReaderId<amethyst::ui::UiEvent>) -> Self {
        Self {
            reader_id,	
        }
    }
}

pub struct SimpleButtonSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, SimpleButtonSystem> for SimpleButtonSystemDesc {
    fn build(self, world: &mut World) -> SimpleButtonSystem {
        use amethyst::shred::SystemData;
        let mut event_channel = <Write<amethyst::shrev::EventChannel<amethyst::ui::UiEvent>>>::fetch(world);
        let reader_id = event_channel.register_reader();

        SimpleButtonSystem::new(reader_id)
    }
}