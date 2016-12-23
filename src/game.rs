extern crate piston;
extern crate opengl_graphics;
extern crate sprite;
extern crate graphics;

use std::rc::Rc;
use std::collections::HashMap;

use sprite::*;
use opengl_graphics::{ Texture, GlGraphics };
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::{ Line, Graphics, DrawState };
use graphics::math::Matrix2d;
use gl;

use piston::input::{ RenderArgs, UpdateArgs };
use piston::input::{ Input };
use cgmath::Point2;

use world::World;
use camera::Camera;
use resource::Resources;
use util;


pub struct Game {
    pub camera: Camera,
    scene: Scene<Texture>,
    world: World,
    resources: Rc<Resources>,
    mouse: Point2<f64>,
}

impl Game {
    pub fn new() -> Game {

        let mut resources = Resources::new();
        let resources_rc = Rc::new(resources);

        let mut scene: Scene<Texture> = Scene::new();
        let mut world = World::new();
        let mut camera = Camera::new();

        Game {
            camera: camera,
            scene: scene,
            world: world,
            resources: resources_rc.clone(),
            mouse: Point2::new(0.0, 0.0),
        }
    }

    pub fn load_scene(&mut self) {
        let mut scene: Scene<Texture> = Scene::new();
        self.scene = scene;
        let level = self.world.get_level();
        let offset_x = 0.0;
        let offset_y = 0.0;

        for i in 0..level.height {
            for j in 0..level.width {
                let index = (i * level.width) + j;
                let mut tile = level.map.get_mut(index).unwrap();
                let mut sprite = match tile.tex_code {
                   0 ... 10 => self.resources.textures.get("dirt1").map(|tx| Sprite::from_texture(tx.clone())).unwrap(),
                   11 ... 18 => self.resources.textures.get("dirt2").map(|tx| Sprite::from_texture(tx.clone())).unwrap(),
                   19 => self.resources.textures.get("dirt1").map(|tx| Sprite::from_texture(tx.clone())).unwrap(),
                   _ => panic!("aahhh")
                };
                let x = (j as f64) * (level.tile_width as f64);
                let y = (i as f64) * (level.tile_height as f64 * 2.0);
                let iso_pt = util::toIso(Point2::new(x, y));
                sprite.set_anchor(0.0, 0.0);
                sprite.set_position(iso_pt.x, iso_pt.y);
                sprite.set_visible(true);
                let id = self.scene.add_child(sprite);
                tile.set_sprite_id(id);

            }
        }
    }

    pub fn draw_grid(&mut self,
                     line: &Line,
                     draw_state: &DrawState,
                     transform: Matrix2d,
                     g: &mut GlGraphics)
    {
        let level = self.world.get_level();
        let width = (level.width * level.tile_width) as f64;
        let height = (level.height * level.tile_height) as f64;
        for i in 0..level.width+1 {
            for j in 0..level.height+1 {
                let h_x1 = (i * level.tile_width) as f64;
                let h_y1 = 0.0;
                let h_x2 = (i * level.tile_width) as f64;
                let h_y2 = height;
                let h_iso1 =  util::toIso(Point2::new(h_x1, h_y1));
                let h_iso2 =  util::toIso(Point2::new(h_x2, h_y2));
                line.draw([h_iso1.x, h_iso1.y, h_iso2.x, h_iso2.y], draw_state, transform, g);

                let v_x1 =  0.0;
                let v_y1 = (j * level.tile_height) as f64;
                let v_x2 = width;
                let v_y2 = (j * level.tile_width) as f64;
                let v_iso1 =  util::toIso(Point2::new(v_x1, v_y1));
                let v_iso2 =  util::toIso(Point2::new(v_x2, v_y2));
                line.draw([v_iso1.x, v_iso1.y, v_iso2.x, v_iso2.y], draw_state, transform, g);
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        gl.draw(args.viewport(), |c, g| {

            let transform = c.transform
                .trans(self.camera.x, self.camera.y)
                .zoom(self.camera.zoom);

            self.scene.draw(transform, g);

            //let line = &Line::new([0.2, 0.2, 0.2, 0.9], 0.5);
            //self.draw_grid(line, &c.draw_state, transform, g);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.camera.update();
        let (x, y) = util::to2DT(self.mouse.x, self.mouse.y);
        let ref level = self.world.get_level();
        let ref mut scene = self.scene;

        /*
        level.tile_for_pointT(x, y)
            .and_then(|tile| tile.sprite_id)
            .and_then(|id| scene.child_mut(id))
            .map(|sprite| sprite.set_visible(false));
        */
        //sprite.set_texture(self.texture2.clone());
    }


    pub fn input(&mut self, input: &Input) {
        use piston::input::Input::{Press, Release, Move};
        use piston::input::Button::{Mouse, Keyboard};
        use piston::input::Key;
        use piston::input::Key::*;
        use piston::input::Motion::MouseCursor;

        match *input {
            Move(MouseCursor(x, y)) =>  {

                let old_tx = self.resources.textures.get("dirt2").unwrap();
                let new_tx = self.resources.textures.get("dirt3").unwrap();

                let ref level = self.world.get_level();
                let ref mut scene = self.scene;

                let old_sid = level.get_view(Point2::new((self.mouse.x - self.camera.x) / self.camera.zoom, (self.mouse.y - self.camera.y) / self.camera.zoom), 30.0, 40.0);
                old_sid.map(|id| scene.child_mut(id).map(|sprite| {
                    //sprite.set_visible(false)
                    sprite.set_texture(old_tx.clone());
                }));

                self.mouse.x = x;
                self.mouse.y = y;

                let sid = level.get_view(Point2::new((x - self.camera.x) / self.camera.zoom, (y - self.camera.y) / self.camera.zoom), 30.0, 40.0);
                sid.map(|id| scene.child_mut(id).map(|sprite| {
                    //sprite.set_visible(false)
                    sprite.set_texture(new_tx.clone());
                }));
            }
            /*
            Press(Mouse(LeftMouseButton)) => {
                println!("Mouse here: '{} {}'", self.mouse.x, self.mouse.y);
                let mut twoDPt = util::to2D(self.mouse);
                println!("2d mouse here: '{} {}'", twoDPt.x, twoDPt.y);
                let level = self.world.get_level();
                let tile = level.tile_for_point(twoDPt).unwrap();
                println!{"tile_id: {}", tile.id};
                let sprite_id = tile.sprite_id.unwrap();
                println!("sprite_id: {}", sprite_id);
                //let sprite = self.scene.child(sprite_id).unwrap();
                //println!("bounding: {}", sprite.bounding_box().get(0).unwrap());
                self.scene.remove_child(sprite_id);
            }
            */

            Press(Keyboard(Left)) | Press(Keyboard(A)) => {
                self.camera.dx = 5.0;
            }
            Press(Keyboard(Right)) | Press(Keyboard(D)) => {
                self.camera.dx = -5.0;
            }
            Press(Keyboard(Up)) | Press(Keyboard(W)) => {
                self.camera.dy = 5.0;
            }
            Press(Keyboard(Down)) | Press(Keyboard(S)) => {
                self.camera.dy = -5.0;
            }

            Release(Keyboard(Left)) | Release(Keyboard(A)) => {
                self.camera.dx = 0.0;
            }
            Release(Keyboard(Right)) | Release(Keyboard(D)) => {
                self.camera.dx = 0.0;
            }
            Release(Keyboard(Up)) | Release(Keyboard(W)) => {
                self.camera.dy = 0.0;
            }
            Release(Keyboard(Down)) | Release(Keyboard(S)) => {
                self.camera.dy = 0.0;
            }

            Press(Keyboard(Equals)) => {
                self.camera.zoom *= 2.0;
            }
            Press(Keyboard(Minus)) => {
                self.camera.zoom /= 2.0;
            }

            Press(Keyboard(R)) => {
                self.camera.zoom = 1.0;
                self.camera.x = 0.0;
                self.camera.y = 0.0;
            }

            Press(Keyboard(H)) => {
                println!("Reloading scene");
                self.load_scene();
            }


            _ => {}
        }
    }
}
