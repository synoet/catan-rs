use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Tile {
    pub resource: Resource,
    pub number: Option<u8>,
    pub vertexes: Vec<Vertex>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
