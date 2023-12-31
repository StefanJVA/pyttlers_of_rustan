use crate::board::Board;
use crate::player::Player;

pub struct GameState {
    board: Board,
    players: Vec<Player>,
    // turn_order, current_player, available_cards, etc.
}
