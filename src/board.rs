use crate::resource::{Resource, Resources};
use std::iter;
use crate::tile::Tile;
use rand::Rng;
pub struct Board {
    tiles: Vec<Tile>,
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
    map: Vec<Vec<Option<Vertex>>>,
}

pub struct Vertex {
    pub x: usize,
    pub y: usize,
}

impl Vertex {
    pub fn new(x: usize, y: usize) -> Vertex {
        Vertex { x, y }
    }
}

impl Clone for Vertex {
    fn clone(&self) -> Vertex {
        Vertex {
            x: self.x,
            y: self.y,
        }
    }
}
pub struct Edge {
    pub vertexes: Vec<Vertex>,
}

impl Board {
    pub fn new() -> Board {
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

        let mut map = vec![vec![None; 16]; 11];


        let mut initial_vertex_column_positions: Vec<usize> = vec![3, 5, 8];


        for x in 0..11 {
            for vertex in initial_vertex_column_positions.iter() {
                map[x][*vertex] = Some(Vertex::new(x, *vertex));
            }
        }



        !unimplemented!();
    }

    pub fn place_settlement(&mut self, player: u8, tiles: Vec<Tile>) {





        
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
