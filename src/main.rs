extern crate piston;
extern crate sdl2;
extern crate sdl2_window;
extern crate fps_counter;
extern crate opengl_graphics;
extern crate graphics;
extern crate gl;
extern crate sprite;
extern crate uuid;
extern crate cgmath;
extern crate rand;


use sdl2_window::{ Sdl2Window, OpenGL };
use piston::window::{ Size, Window, AdvancedWindow, OpenGLWindow, WindowSettings };
use piston::event_loop::{ Events, EventLoop };
use opengl_graphics::{ GlGraphics, Texture, TextureSettings };
use cgmath::Point2;

use game::Game;
use hud::Hud;


mod game;
mod world;
mod camera;
mod hud;
mod resource;
mod util;

pub fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Sdl2Window = WindowSettings::new(
        "Midnight",
        Size { width: 1024, height: 768 })
        .samples(8)
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter = fps_counter::FPSCounter::new();

    let mut game = Game::new();
    let mut hud = Hud::new();
    game.load_scene();

    let mut events = window.events().ups(60).max_fps(60);
    while let Some(e) = events.next(&mut window) {
        use piston::input::Event;

        match e {
            Event::Render(args) => {
                let fps = fps_counter.tick();
                let title = format!(
                    "Midnight FPS: {}, CAMERA: {} {}",
                    fps,
                    game.camera.x,
                    game.camera.y
                );
                window.set_title(title);

                game.render(&args, &mut gl);
                hud.draw(&args, &mut gl);
            }
            Event::Update(args) => {
                game.update(&args);
                hud.update(&game)
            }
            Event::Input(ref input) => {
                game.input(&input);
                hud.input(&input);
            }
            _ => {}
        }
    }
}
