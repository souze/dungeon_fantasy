use std::collections::HashMap;

use amethyst::ecs::Entity;

use crate::systems::combat::SpellTemplate;

#[derive(Debug)]
pub struct ButtonMap {
    pub map: HashMap<Entity, ButtonAction>,
}

#[derive(Debug, Clone)]
pub enum ButtonAction {
    CastSpell {
        spell_template: SpellTemplate,
    },
    Nothing,
}