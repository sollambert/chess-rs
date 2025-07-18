use crate::game::Game;

mod board;
mod game;
mod pieces;
mod tests;


fn main() {
    let mut game = Game::new(3600);
}