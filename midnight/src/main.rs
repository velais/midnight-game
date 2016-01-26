extern crate piston;
extern crate sdl2;
extern crate sdl2_window;
extern crate fps_counter;

use sdl2_window::Sdl2Window;
use piston::window::{ Size, Window, AdvancedWindow, OpenGLWindow, WindowSettings };
use piston::event_loop::{ Events, EventLoop };

pub fn main() {

    let mut fps_counter = fps_counter::FPSCounter::new();

    let mut window: Sdl2Window = WindowSettings::new(
        "Hello Piston!", 
        Size { width: 640, height: 480 })
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = window.events().ups(60).max_fps(100);
    while let Some(e) = events.next(&mut window) {
        use piston::input::Event;

        match e {
            Event::Render(_) => {
                let fps = fps_counter.tick();
                let title = format!(
                    "Midnight FPS: {}",
                    fps);
                window.set_title(title);
            }
            Event::AfterRender(_) => {}
            Event::Update(_) => {}
            _ => {}
        }
    }
}
