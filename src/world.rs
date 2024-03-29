use uuid::Uuid;
use cgmath::Point2;
use rand::{thread_rng, Rng};
use util;
use std::num::Wrapping;

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
    pub tile_width: usize,
    pub tile_height: usize,
    pub width: usize,
    pub height: usize
}

impl Level {
    pub fn new() -> Level {
        let width = 40;
        let height = 40;
        let mut map = Vec::with_capacity(width * height);
        for i in 0..(width * height) {
            map.push(Tile::new());
        }
        Level {
            map: map,
            tile_width: 32,
            tile_height: 16,
            width: width,
            height: height
        }
    }

    pub fn tile_for_point(&self, pt: Point2<f64>) -> Option<&Tile> {
        let col = (pt.x as usize / self.tile_width) as usize;
        let row = (pt.y as usize / self.tile_height) as usize;
        let index = (row * self.width) + col;
        self.map.get(index)
    }

    pub fn tile_for_pointT(&self, x: f64, y: f64) -> Option<&Tile> {
        let col = (x as usize / self.tile_width) as usize;
        let row = (y as usize / self.tile_height / 2) as usize;
        let index = (row.wrapping_mul(self.height)).wrapping_add(col);
        self.map.get(index as usize)
    }

    pub fn get_view(&self, pt: Point2<f64>, width: f64, height: f64) -> Option<Uuid> {
        let map_pt = util::to2D(Point2::new(pt.x - 5.0, pt.y));
        let x = map_pt.x; //% self.tile_width as f64;
        let y = map_pt.y; //% self.tile_height as f64;
        let tile = self.tile_for_pointT(x, y);
        //println!("{} {}", x, y);
        tile.and_then(|t| t.sprite_id)
    }
}

pub struct Tile {
    pub id: Uuid,
    pub sprite_id: Option<Uuid>,
    pub tex_code: usize,
}

impl Tile {
    pub fn new() -> Tile {
        let mut rng = thread_rng();
        let tex_code: usize = rng.gen_range(0, 19);
        Tile {
            id: Uuid::new_v4(),
            sprite_id: None,
            tex_code: tex_code
        }
    }

    pub fn set_sprite_id(&mut self, id: Uuid) {
        self.sprite_id = Some(id);
    }
}
