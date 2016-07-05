extern crate find_folder;

use std::collections::HashMap;
use std::rc::Rc;

use opengl_graphics::{ Texture };
use opengl_graphics::glyph_cache::GlyphCache;

pub struct Resources {
    pub font: GlyphCache<'static>,
    pub textures: HashMap<&'static str, Rc<Texture>>
}

impl Resources {
    pub fn new() -> Resources {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

        let texture_1 = Rc::new(Texture::from_path(assets.join("dirt_1.png")).unwrap());
        let texture_2 = Rc::new(Texture::from_path(assets.join("dirt_2.png")).unwrap());
        let texture_3 = Rc::new(Texture::from_path(assets.join("dirt_3.png")).unwrap());

        let mut textures = HashMap::new();
        textures.insert("dirt1", texture_1.clone());
        textures.insert("dirt2", texture_2.clone());
        textures.insert("dirt3", texture_3.clone());

        Resources {
            font: GlyphCache::new(assets.join("fonts/SourceCodePro-Regular.ttf")).unwrap(),
            textures: textures
        }
    }
}