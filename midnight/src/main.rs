extern crate piston;
extern crate sdl2;
extern crate sdl2_window;
extern crate fps_counter;
extern crate opengl_graphics;
extern crate graphics;
extern crate sprite;
extern crate uuid;


use sdl2_window::{ Sdl2Window, OpenGL };
use piston::window::{ Size, Window, AdvancedWindow, OpenGLWindow, WindowSettings };
use piston::event_loop::{ Events, EventLoop };
use opengl_graphics::{ GlGraphics, Texture, TextureSettings };
use game::Game;


mod game;
mod world;

pub fn main() {
    let opengl = OpenGL::V3_2;

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

    let mut cursor = [0.0, 0.0];

    let mut events = window.events().ups(60).max_fps(100);
    while let Some(e) = events.next(&mut window) {
        use piston::input::Event;
        use piston::input::Input::{ Press };
        use piston::input::Button::{ Mouse };
        use piston::input::MouseButton::{ Left };
        use piston::input::mouse::MouseCursorEvent;


        match e {
            Event::Render(args) => {
                let fps = fps_counter.tick();
                let title = format!(
                    "Midnight FPS: {}",
                    fps);
                window.set_title(title);

                game.render(&args, &mut gl);
            }
            Event::AfterRender(_) => {}
            Event::Update(args) => {
                game.update(&args);
            }
            Event::Input(Press(Mouse(LeftMouseButton))) => {
                println!("Mouse at: '{} {}'", cursor.get(0).unwrap(), cursor.get(1).unwrap());
            }
            _ => {}
        }
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
        });
    }
}
