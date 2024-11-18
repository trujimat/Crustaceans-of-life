use crustaceans_of_life::constants::{COLS, ROWS};
use crustaceans_of_life::game_state::GameState;
use ggez::*;

fn main() {
    // Initialize the game state with configuration
    let game_state = GameState::new(ROWS, COLS);

    // Set up the ggez context and event loop
    let (ctx, event_loop) = game_state.setup_window();

    // Run the game
    event::run(ctx, event_loop, game_state);
}
