use crate::constants::StructureType;
use crate::constants::TileType;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct TileCoord {
    q: isize,
    r: isize,
}
pub type StreetCoord = (TileCoord, TileCoord);
pub type StructureCoord = (TileCoord, TileCoord, TileCoord);

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    token: u32,
}

#[derive(Debug)]
pub struct Street {
    owner: u32, // ID of the player who owns the structure
}

#[derive(Debug)]
pub struct Structure {
    owner: u32, // ID of the player who owns the structure
    structure_type: StructureType,
}

pub struct Board {
    tiles: HashMap<TileCoord, Tile>,
    streets: HashMap<StreetCoord, Street>,
    structures: HashMap<StructureCoord, Structure>,
}

// impl Board {
//     pub fn new(asdf: u32) -> Self {
//         println!("asdf")
//     }
// }
