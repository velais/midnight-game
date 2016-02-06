extern crate piston;
extern crate opengl_graphics;
extern crate find_folder;
extern crate sprite;
extern crate graphics;

use std::rc::Rc;

use sprite::*;
use piston::input::{ RenderArgs, UpdateArgs };
use opengl_graphics::{ Texture, GlGraphics };

use world::World;


pub struct Game {
    scene: Scene<Texture>,
    world: World,
    texture: Rc<Texture>
}

impl Game {
    pub fn new() -> Game {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let cube_art = assets.join("cube.png");
        let cube_texture = Rc::new(Texture::from_path(cube_art).unwrap());

        let mut scene: Scene<Texture> = Scene::new();
        let mut world = World::new();

        let width = 10.0;
        let level = world.get_level();
        let mut index = 0.0;
        let offset_x = 200.0;
        let offset_y = -50.0;
        for i in 0..48 {
            for j in 0..48 {
                let tile = level.map.get(i * j);
                let mut sprite = Sprite::from_texture(cube_texture.clone());
                let x = (j as f64) * 30.0;
                let y = (i as f64) * 30.0;
                let iso_x = x - y;
                let iso_y = (x + y) / 2.0;
                sprite.set_position((iso_x / 2.0)+offset_x, (iso_y / 2.0)+offset_y);
                scene.add_child(sprite);
            }
        }

        Game {
            scene: scene,
            world: World::new(),
            texture: cube_texture
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        gl.draw(args.viewport(), |c, g| {
            clear([0.8, 0.8, 0.3, 1.0], g);
            self.scene.draw(c.transform, g);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn input(&mut self) {

    }
}
