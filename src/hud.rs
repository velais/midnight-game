extern crate find_folder;
extern crate fps_counter;

use std::rc::Rc;
use std::collections::HashMap;

use opengl_graphics::{ GlGraphics };
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::math::{ Matrix2d };
use graphics::*;
use piston::input::{ Input, RenderArgs };

use game::Game;
use camera::Camera;
use resource::Resources;

pub struct Hud {
    pub x: f64,
    pub y: f64,
    pub visible: bool,
    resources: Resources,
    fps: fps_counter::FPSCounter,
    zoom: f64
}

impl Hud {
    pub fn new() -> Hud {
        Hud {
            x: 0.0,
            y: 0.0,
            visible: true,
            resources: Resources::new(),
            fps: fps_counter::FPSCounter::new(),
            zoom: 1.0
        }
    }

    pub fn update(&mut self, game: &Game) {
        self.zoom = game.camera.zoom;
    }

    pub fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let fps = self.fps.tick();

        if !self.visible {
            return
        }

        gl.draw(args.viewport(), |c, g| {
            let mx2d = c.trans(10.0, 10.0);

            let bg = Rectangle::new([0.4, 0.4, 0.4, 0.8]);
            bg.draw([0.0, 0.0, 120.0, 80.0], &c.draw_state, mx2d.transform, g);

            let mut text = Text::new(12);
            text.draw(&format!("fps: {}", fps),
                      &mut self.resources.font,
                      &c.draw_state,
                      mx2d.trans(5.0, 15.0).transform,
                      g);

            text.draw(&format!("ups: 60"),
                      &mut self.resources.font,
                      &c.draw_state,
                      mx2d.trans(5.0, 27.0).transform,
                      g);

            text.draw(&format!("zoom: {}", self.zoom),
                      &mut self.resources.font,
                      &c.draw_state,
                      mx2d.trans(5.0, 39.0).transform,
                      g);
        });
    }

    pub fn input(&mut self, input: &Input) {
        use piston::input::Input::{Press, Release, Move};
        use piston::input::Button::{Mouse, Keyboard};
        use piston::input::Key;
        use piston::input::Key::*;
        use piston::input::Motion::MouseCursor;

        match *input {
            Press(Keyboard(Z)) => {
                println!("Toggle hud!");
                self.visible = !self.visible;
            }

            _ => {}
        }
    }
}
