use chess::{cli::CLIDisplayer, game, game_controller::GameController};

fn main() {
    let mut my_controller = GameController::new(game::Game::from_standard_board(), &CLIDisplayer);
    my_controller.run();
}
