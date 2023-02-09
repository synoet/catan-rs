#[derive(Debug)]
pub enum Resource {
    Wood,
    Ore,
    Brick,
    Wheat,
    Desert,
    Sheep,
}

pub struct Resources {
    pub wood: u8,
    pub ore: u8,
    pub brick: u8,
    pub wheat: u8,
    pub sheep: u8,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            wood: 0,
            ore: 0,
            brick: 0,
            wheat: 0,
            sheep: 0,
        }
    }
}

impl Resource {
    pub fn from_number(n: u8) -> Resource {
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
