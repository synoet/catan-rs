pub mod board;
pub mod game;
pub mod player;
pub mod resource;
pub mod tile;

use game::Game;
use player::Player;

fn main() {
    let _game = Game::new(vec![
        Player::new(String::from("Teo")),
        Player::new(String::from("Gabriella")),
        Player::new(String::from("Peenz")),
    ]);
}
