extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const WIDTH: i32 = 15;
const HEIGHT: i32 = WIDTH;

fn main() {
    let mut piston_window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game: Game = Game::new(WIDTH, HEIGHT);
    while let Some(event) = piston_window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        piston_window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
