extern crate find_folder;

use std::collections::HashMap;
use std::rc::Rc;
use image::{ self, DynamicImage };

use opengl_graphics::{ Texture, TextureSettings, Filter};
use opengl_graphics::glyph_cache::GlyphCache;

pub struct Resources {
    pub font: GlyphCache<'static>,
    pub textures: HashMap<&'static str, Rc<Texture>>
}

impl Resources {
    pub fn new() -> Resources {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

        let ref_texture = |name: &str| {
            let path = assets.join(name);
            match Resources::texture_from_path(path) {
                Ok(texture) => Rc::new(texture),
                Err(e) => panic!("Could not load texture {}!", name)
            }
        };

        let mut textures = HashMap::new();
        textures.insert("dirt1", ref_texture("dirt_1.png"));
        textures.insert("dirt2", ref_texture("dirt_2.png"));
        textures.insert("dirt3", ref_texture("dirt_3.png"));

        Resources {
            font: GlyphCache::new(assets.join("fonts/SourceCodePro-Regular.ttf")).unwrap(),
            textures
        }
    }


    fn texture_from_path<P>(path: P) -> Result<Texture, String> where P: AsRef<std::path::Path> {
        let path = path.as_ref();

        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => return Err(format!("Could not load '{:?}': {:?}",
                path.file_name().unwrap(), e)),
        };

        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba()
        };

        Ok(Texture::from_image(&img, &TextureSettings::new()
            .mag(Filter::Nearest)
            .min(Filter::Nearest)
            .mipmap(Filter::Nearest)))
    }
}