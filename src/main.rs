extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use crate::draw::to_coord_u32;
use crate::game::Game;

use piston_window as pw;
use piston_window::{PressEvent, UpdateEvent};

const BACK_COLOR: pw::types::Color = [0.5, 0.5, 0.5, 1.0];
const WIDTH: i32 = 15;
const HEIGHT: i32 = WIDTH;

fn main() {
    let mut piston_window: pw::PistonWindow =
        pw::WindowSettings::new("Snake", [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game: Game = Game::new(WIDTH, HEIGHT);
    while let Some(event) = piston_window.next() {
        if let Some(pw::Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        piston_window.draw_2d(&event, |c, g, _| {
            pw::clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
