use uuid::Uuid;

pub struct World {
    levels: Vec<Level>
}

impl World {
    pub fn new() -> World {
        let level = Level::new();
        World {
            levels: vec![level]
        }
    }

    pub fn get_level(&mut self) -> &Level {
        self.levels.first().unwrap()
    }
}

pub struct Level {
    pub map: Vec<Tile>
}

impl Level {
    pub fn new() -> Level {
        let mut map = Vec::with_capacity(10);
        for i in 0..2304 {
            map.push(Tile::new());
        }
        Level {
            map: map
        }
    }
}

pub struct Tile {
    id: Uuid
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id: Uuid::new_v4()
        }
    }
}