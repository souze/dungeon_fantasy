use amethyst::ecs::{Component, DenseVecStorage};


#[derive(Debug)]
pub struct Monster {
    pub id: MonsterIdentifier,
    pub attack: u32,
    pub health: CurrMax,
}

impl Component for Monster {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct CurrMax {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MonsterIdentifier {
    id: u32,
}

impl MonsterIdentifier {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitReference {
    Player,
    Monster {
        id: MonsterIdentifier
    },
}
pub struct TurnOrder {
    list: Vec<UnitReference>,
}

impl TurnOrder {
    pub fn new(list: Vec<UnitReference>) -> Self {
        Self{ list }
    }

    pub fn advance(&mut self) {
        self.list.rotate_left(1);
    }

    pub fn current(&self) -> UnitReference {
        self.list[0].clone()
    }
}

pub struct Player {
    pub health: CurrMax,
}