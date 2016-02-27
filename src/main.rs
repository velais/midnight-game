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
use game::Game;
use cgmath::Point2;


mod game;
mod world;
mod camera;
mod util;

pub fn main() {
    let opengl = OpenGL::V4_0;

    let mut window: Sdl2Window = WindowSettings::new(
        "Midnight",
        Size { width: 640, height: 480 })
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter = fps_counter::FPSCounter::new();

    let mut game = Game::new();
    game.load_scene();

    let mut events = window.events().ups(60).max_fps(10000);
    while let Some(e) = events.next(&mut window) {
        use piston::input::Event;

        match e {
            Event::Render(args) => {
                let fps = fps_counter.tick();
                let title = format!(
                    "Midnight FPS: {}",
                    fps);
                window.set_title(title);

                game.render(&args, &mut gl);
            }
            Event::Update(args) => {
                game.update(&args);
            }
            Event::Input(ref input) => {
                game.input(&input);
            }
            _ => {}
        }
    }
}