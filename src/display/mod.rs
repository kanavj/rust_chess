use crate::game::Game;
use crate::moves::Move;
pub mod cli;
pub mod gui;

pub trait Display {
    async fn display(&mut self, game: &Game);
    fn user_input(&mut self, game: &Game) -> Option<Move>;
    fn display_message(&self, message: String);
}
