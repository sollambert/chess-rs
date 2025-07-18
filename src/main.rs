use text_io::read;

use crate::game::Game;

mod board;
mod game;
mod pieces;
mod tests;


fn main() {
    print!("Welcome to Chess-rs, please enter desired game time in seconds:");
    let time: u32 = read!();
    let mut game = Game::new(time);
    game.start()
}