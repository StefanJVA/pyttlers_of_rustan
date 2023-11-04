mod board;
mod constants;
mod game_state;
mod player;

use board::Board;
use constants::CardType;
use constants::ResourceType;
use constants::TileType;

fn main() {
    let card = CardType::Knight;
    let tile = TileType::Desert;
    let resource = ResourceType::Wheat;
    let board = Board;

    println!("hello world {:?}", card);
    println!("hello world {:?}", tile);
    println!("hello world {:?}", resource);
    println!("hello world {:?}", board);
}
