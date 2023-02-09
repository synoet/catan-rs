use crate::resource::Resources;
pub struct Player {
    name: String,
    points: u8,
    resources: Resources,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            points: 0,
            resources: Resources {
                wood: 0,
                ore: 0,
                brick: 0,
                wheat: 0,
                sheep: 0,
            },
        }
    }
}
