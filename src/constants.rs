#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StructureType {
    Settlement,
    City,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResourceType {
    Wheat,
    Wood,
    Brick,
    Ore,
    Sheep,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Forest,   // Produces Wood
    Hill,     // Produces Brick
    Pasture,  // Produces Sheep
    Mountain, // Produces Ore
    Field,    // Produces Wheat
    Desert,   // Produces Nothing
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardType {
    Knight,
    VictoryPoint,
    RoadBuilding,
    Monopoly,
    YearOfPlenty,
}
