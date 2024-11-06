use crate::conf::FullscreenType;
use conf::{WindowMode, WindowSetup};
use crustaceans_of_life::constants::{CELL_SIZE, MARGIN};
use crustaceans_of_life::game_state::GameState;
use ggez::conf::NumSamples;
use ggez::*;
fn main() {
    let rows = 3;
    let cols = 3;
    let some_initial_config: &[&[u8]] = &[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]];
    // let some_other_initial_config: &[&[u8]] = &[&[0, 1, 0], &[0, 1, 0], &[1, 1, 1]];

    let game_state = GameState::new(rows, cols, some_initial_config);
    let c = conf::Conf::new();
    let mode_for_the_window = WindowMode {
        width: (2.0 * MARGIN) + (rows as f32 * CELL_SIZE),
        height: (2.0 * MARGIN) + (cols as f32 * CELL_SIZE),
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };
    let window_set_up = WindowSetup {
        title: "The game of life".to_owned(),
        samples: NumSamples::One,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let mut c_copy = c.window_mode(mode_for_the_window);
    c_copy.window_setup = window_set_up;
    let (ctx, event_loop) = ContextBuilder::new("crustaceans of life", "trujimat")
        .default_conf(c_copy)
        .build()
        .unwrap();
    event::run(ctx, event_loop, game_state);
}
