use uuid::Uuid;
use cgmath::Point2;

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

    pub fn get_level(&mut self) -> &mut Level {
        self.levels.first_mut().unwrap()
    }
}

pub struct Level {
    pub map: Vec<Tile>,
    pub tile_size: usize,
    pub width: usize,
    pub height: usize
}

impl Level {
    pub fn new() -> Level {
        let width = 12;
        let height = 12;
        let mut map = Vec::with_capacity(10);
        for i in 0..(width * height) {
            map.push(Tile::new());
        }
        Level {
            map: map,
            tile_size: 30,
            width: width,
            height: height

        }
    }

    pub fn tile_for_point(&self, pt: Point2<f64>) -> &Tile {
        println!("tile for '{} {}'", pt.x, pt.y);
        let col = (pt.x as usize / self.tile_size) as usize;
        let row = (pt.y as usize / self.tile_size) as usize;
        println!("col: {} row: {}", col, row);
        let index = (row * self.width) + col;
        println!("index: {}", index);

        println!("map: {}", self.map.len());
        self.map.get(index).unwrap()
    }
}

pub struct Tile {
    pub id: Uuid,
    pub sprite_id: Option<Uuid>
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id: Uuid::new_v4(),
            sprite_id: None
        }
    }

    pub fn set_sprite_id(&mut self, id: Uuid) {
        self.sprite_id = Some(id);
    }
}
