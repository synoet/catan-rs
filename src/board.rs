use crate::player::Player;
use crate::resource::{Resource, Resources};
use crate::tile::{Tile, Vertex};
use rand::Rng;
use std::collections::HashMap;

pub struct Board {
    tiles: Vec<Tile>,
    spots: Vec<Spot>,
}

#[derive(Debug, Clone)]
pub struct Spot {
    pub x: usize,
    pub y: usize,
    pub tiles: Vec<Tile>,
    pub neighbors: Vec<Spot>,
}

impl Board {
    pub fn new() -> Board {
        let mut rng = rand::thread_rng();
        let mut tiles: Vec<Tile> = vec![];
        let mut possible_numbers: Vec<u8> =
            vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        let mut num_resources_map =
            vec![0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5];

        let tile_top_vertices_indexes: Vec<Vec<usize>> = vec![
            vec![4, 6, 8],
            vec![3, 5, 7, 9],
            vec![2, 4, 6, 8, 10],
            vec![3, 5, 7, 9],
            vec![4, 6, 8],
        ];

        for i in 0..5 {
            for j in 0..tile_top_vertices_indexes[i].len() {
                let resource: Resource = Resource::from_number(
                    num_resources_map.remove(rng.gen_range(0..num_resources_map.len())),
                );
                let number: Option<u8> = match resource {
                    Resource::Desert => None,
                    _ => Some(possible_numbers.remove(rng.gen_range(0..possible_numbers.len()))),
                };

                let current_row = i * 3;

                tiles.push(Tile {
                    resource,
                    number,
                    vertexes: vec![
                        // Top vertex
                        Vertex {
                            x: tile_top_vertices_indexes[i][j],
                            y: current_row,
                        },
                        // Top Left Vertex
                        Vertex {
                            x: tile_top_vertices_indexes[i][j] - 1,
                            y: current_row + 1,
                        },
                        // Bottom Left Vertex
                        Vertex {
                            x: tile_top_vertices_indexes[i][j] - 1,
                            y: current_row + 2,
                        },
                        // Bottom Vertex
                        Vertex {
                            x: tile_top_vertices_indexes[i][j],
                            y: current_row + 3,
                        },
                        // Top Right vertex
                        Vertex {
                            x: tile_top_vertices_indexes[i][j] + 1,
                            y: current_row + 1,
                        },
                        // Top Right vertex
                        Vertex {
                            x: tile_top_vertices_indexes[i][j] + 1,
                            y: current_row + 2,
                        },
                    ],
                });
            }
        }

        let mut all_vertexes: HashMap<String, Vertex> = HashMap::new();

        for tile in &tiles {
            for vertex in &tile.vertexes {
                if !all_vertexes.contains_key(&vertex.to_string()) {
                    all_vertexes.insert(vertex.to_string(), vertex.clone());
                }
            }
        }

        let mut spots: Vec<Spot> = vec![];

        for (key, vertex) in all_vertexes {
            let mut touching_tiles: Vec<Tile> = vec![];
            for tile in &tiles {
                for tile_vertex in &tile.vertexes {
                    if tile_vertex.to_string() == key {
                        touching_tiles.push(tile.clone());
                    }
                }
            }

            spots.push(Spot {
                x: vertex.x,
                y: vertex.y,
                tiles: touching_tiles,
                neighbors: vec![],
            });
        }

        for spot in &spots {
            for possible_neighbor in &spots {
                if possible_neighbor.x >= spot.x - 1
                    && possible_neighbor.x <= spot.x + 1
                    && possible_neighbor.y >= spot.y - 1
                    && possible_neighbor.y <= spot.y + 1
                {
                    if possible_neighbor.x != spot.x || possible_neighbor.y != spot.y {
                        spot.neighbors.push(possible_neighbor.clone());
                    }
                }
            }
        }

        Board { tiles, spots }
    }

    pub fn draw(&self) {
        let mut map = vec![vec!["___"; 12]; 16];

        for tile in &self.tiles {
            println!("{:?}", tile);
        }

        for tile in &self.tiles {
            for vertex in &tile.vertexes {
                map[vertex.y][vertex.x] = " x ";
            }
        }

        for row in map {
            for col in row {
                print!("{} ", col);
            }
            println!("");
        }
    }
}
