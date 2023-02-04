use rand::Rng;
struct Board {
    tiles: Vec<Tile>,
}

struct Player {
    points: u8,
    resources: Vec<Resource>,
}

#[derive(Debug)]
struct Tile {
    resource: Resource,
    number: Option<u8>,
}

#[derive(Debug)]
enum Resource {
    Wood,
    Ore,
    Brick,
    Wheat,
    Desert,
    Sheep,
}

impl Resource {
    fn from_number(n: u8) -> Resource {
        match n {
            0 => Resource::Wood,
            1 => Resource::Ore,
            2 => Resource::Brick,
            3 => Resource::Wheat,
            4 => Resource::Sheep,
            5 => Resource::Desert,
            _ => panic!("Invalid resource"),
        }
    }
}

impl Board {
    fn new() -> Board {
        let mut rng = rand::thread_rng();
        let mut tiles: Vec<Tile> = vec![];
        let mut possible_numbers: Vec<u8> =
            vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        let mut num_resources_map =
            vec![0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5];

        for _ in 0..18 {
            let resource: Resource = Resource::from_number(
                num_resources_map.remove(rng.gen_range(0..num_resources_map.len())),
            );
            let number: Option<u8> = match resource {
                Resource::Desert => None,
                _ => Some(possible_numbers.remove(rng.gen_range(0..possible_numbers.len()))),
            };

            tiles.push(Tile { resource, number });
        }

        Board { tiles }
    }

    fn print(&self) {
        for i in 0..2 {
            print!(
                "{:?}-{}  ",
                self.tiles[i].resource,
                self.tiles[i].number.unwrap_or(0)
            );
        }
        println!("");
        for i in 2..6 {
            print!(
                "{:?}-{}  ",
                self.tiles[i].resource,
                self.tiles[i].number.unwrap_or(0)
            );
        }

        println!("");
        for i in 6..11 {
            print!(
                "{:?}-{}  ",
                self.tiles[i].resource,
                self.tiles[i].number.unwrap_or(0)
            );
        }

        println!("");
        for i in 11..15 {
            print!(
                "{:?}-{}  ",
                self.tiles[i].resource,
                self.tiles[i].number.unwrap_or(0)
            );
        }

        println!("");
        for i in 15..18 {
            print!(
                "{:?}-{}  ",
                self.tiles[i].resource,
                self.tiles[i].number.unwrap_or(0)
            );
        }
    }
}

fn main() {
    let mut board: Board = Board::new();
    board.print()
}
