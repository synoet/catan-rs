use crate::board::Board;
use crate::player::Player;
use crate::resource::Resources;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct Game {
    pub players: Vec<Player>,
    pub resources: Resources,
    pub turn: u8,
    pub board: Board,
    pub order: Vec<u8>,
}

impl Game {
    fn generate_random_order(len: u8) -> Vec<u8> {
        let mut order: Vec<u8> = (0..len).collect::<Vec<u8>>();
        order.shuffle(&mut thread_rng());
        order
    }

    pub fn new(players: Vec<Player>) -> Game {
        let num_players: u8 = players.len() as u8;

        Game {
            players,
            resources: Resources::new(),
            turn: 0,
            board: Board::new(),
            order: Game::generate_random_order(num_players),
        }
    }

    pub fn roll() -> u8 {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..=6) + rng.gen_range(1..=6);
        roll
    }

    pub fn process_roll(&mut self, roll: u8, player: Player) {
        match roll {
            7 => {
                // [TODO]: make user move the robber 
                unimplemented!();
            }
            _ => {
                // [TODO]: make user choose which resource to take
                unimplemented!();

            }
        }
    }
}
