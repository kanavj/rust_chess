use chess::{game::Game, game_controller::GameController, gui::GUIDisplayer};

#[macroquad::main("Chess")]
async fn main() {
    let displayer = GUIDisplayer::new(8, 8).await;
    let mut my_controller = GameController::new(Game::from_standard_board(), displayer);
    my_controller.run().await;
}
