#![allow(unused)]

mod game;

use game::models::{Game, Board};

fn main() {
    let p1 = "Player 1".to_string();
    let p2 = "Player 2".to_string();

    let mut game = Game {
        is_over: false,
        winner: None,
        player_b: p2,
        player_w: p1,
        board: Board::new(),
    };

    print!("{}", game.board);
}
