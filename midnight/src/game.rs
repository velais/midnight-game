extern crate piston;
extern crate opengl_graphics;
extern crate find_folder;
extern crate sprite;
extern crate graphics;

use std::rc::Rc;

use sprite::*;
use opengl_graphics::{ Texture, GlGraphics };

use piston::input::{ RenderArgs, UpdateArgs };
use piston::input::{ Input };
use cgmath::Point2;

use world::World;
use util;


pub struct Game {
    scene: Scene<Texture>,
    world: World,
    texture: Rc<Texture>,
}

impl Game {
    pub fn new() -> Game {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let cube_art = assets.join("cube.png");
        let cube_texture = Rc::new(Texture::from_path(cube_art).unwrap());

        let mut scene: Scene<Texture> = Scene::new();
        let mut world = World::new();

        Game {
            scene: scene,
            world: world,
            texture: cube_texture
        }
    }

    pub fn load_scene(&mut self) {
        let level = self.world.get_level();
        let offset_x = 0.0;
        let offset_y = 0.0;

        for i in 0..level.height {
            for j in 0..level.width {
                let index = (i * level.width) + j;
                let mut tile = level.map.get_mut(index).unwrap();
                let mut sprite = Sprite::from_texture(self.texture.clone());
                let x = (j as f64) * level.tile_size as f64;
                let y = (i as f64) * level.tile_size as f64;
                let iso_pt = util::toIso(Point2::new(x, y));
                sprite.set_anchor(0.0, 0.0);
                sprite.set_position((iso_pt.x)+offset_x, (iso_pt.y)+offset_y);
                let id = self.scene.add_child(sprite);
                tile.set_sprite_id(id);

            }
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
    

    pub fn input(&mut self, input: &Input, cursor: &Point2<f64>) {
        use piston::input::Input::Press;
        use piston::input::Button::Mouse;
        use piston::input::MouseButton::Left;

        match *input {
            Press(Mouse(LeftMouseButton)) => {
                println!("Mouse here: '{} {}'", cursor.x, cursor.y);
                let mut twoDPt = util::to2D(*cursor);
                println!("2d mouse here: '{} {}'", twoDPt.x, twoDPt.y);
                let level = self.world.get_level();
                let tile = level.tile_for_point(twoDPt);
                println!{"tile_id: {}", tile.id};
                let sprite_id = tile.sprite_id.unwrap();
                println!("sprite_id: {}", sprite_id);
                self.scene.remove_child(sprite_id);
            }
            _ => {}
        }
    }
}
