use crate::constants::StructureType;
use crate::constants::TileType;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct TileCoord {
    q: isize,
    r: isize,
}
type StreetCoord = (TileCoord, TileCoord);
type StructureCoord = (TileCoord, TileCoord, TileCoord);

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    token: u8,
}

#[derive(Debug)]
pub struct Street {
    owner: u8, // ID of the player who owns the structure
}

#[derive(Debug)]
pub struct Structure {
    owner: u8, // ID of the player who owns the structure
    structure_type: StructureType,
}

pub struct Board {
    tiles: HashMap<TileCoord, Tile>,
    streets: HashMap<StreetCoord, Street>,
    structures: HashMap<StructureCoord, Structure>,
}
