use std::collections::HashMap;

use crate::cards::CardType;
use crate::resources::ResourceType;

pub struct Player {
    ressources: HashMap<ResourceType, u32>,
    cards: HashMap<CardType, u32>,
}
