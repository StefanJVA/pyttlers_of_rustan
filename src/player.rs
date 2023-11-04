use crate::board::StreetCoord;
use crate::board::StructureCoord;
use crate::constants::CardType;
use crate::constants::ResourceType;
use crate::constants::StructureType;
use std::collections::HashMap;

pub struct Player {
    id: u32,
    ressources: HashMap<ResourceType, u32>,
    cards: HashMap<CardType, u32>,
    streets: Vec<StreetCoord>,
    structures: HashMap<StructureCoord, StructureType>,
}
