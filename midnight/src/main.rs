extern crate piston_window;

use piston_window::*;

fn main() {
    println!("Hello, Midnight!");

    let window: PistonWindow = 
        WindowSettings::new("Hello Midnight!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], 
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
