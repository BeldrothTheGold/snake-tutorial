use piston_window::{*, types::Color};
use crate::game::Game;

mod snake;
mod draw;
mod game;

const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

const GAME_WIDTH: u32 = 20;
const GAME_HEIGHT: u32 = 20;

fn main() {
    let (gui_x, gui_y) = draw::to_coord(GAME_WIDTH, GAME_HEIGHT);
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new(
        "RSnake",
        [gui_x, gui_y],
    ).exit_on_esc(true)
    .build().unwrap();

    let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c,g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
