extern crate piston;
extern crate opengl_graphics;
extern crate find_folder;
extern crate sprite;
extern crate graphics;

use std::rc::Rc;

use sprite::*;
use piston::input::{ RenderArgs, UpdateArgs };
use opengl_graphics::{ Texture, GlGraphics };


pub struct Game {
    scene: Scene<Texture>
}

impl Game {
    pub fn new() -> Game {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let guy = assets.join("cube.png");
        let guy = Rc::new(Texture::from_path(guy).unwrap());

        let mut cubesprite = Sprite::from_texture(guy);
        cubesprite.set_position(100.0, 100.0);

        let mut scene: Scene<Texture> = Scene::new();
        let id = scene.add_child(cubesprite);

        Game {
            scene: scene
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
