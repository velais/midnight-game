extern crate find_folder;

use std::rc::Rc;

use opengl_graphics::{ GlGraphics };
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::math::{ Matrix2d };
use graphics::*;

use resource::Resources;

pub struct Hud {
    pub x: f64,
    pub y: f64,
    resources: Resources
}

impl Hud {
    pub fn new() -> Hud {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        Hud {
            x: 0.0,
            y: 0.0,
            resources: Resources{ font: GlyphCache::new(assets.join("fonts/SourceCodePro-Regular.ttf")).unwrap() }
        }
    }

    pub fn update(&mut self) {
    }

    pub fn draw(&mut self, c: Context, g: &mut GlGraphics) {
        let bg = Rectangle::new([1.0, 4.0, 4.0, 1.0]);
        bg.draw([0.0, 0.0, 120.0, 80.0], &c.draw_state, c.trans(10.0, 5.0).transform, g);

        let mut text = Text::new(20);
        text.draw("Midnight",
                  &mut self.resources.font,
                  &c.draw_state,
                  c.trans(15.0, 35.0).transform,
                  g);
    }

}
