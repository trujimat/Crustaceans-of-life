use crate::conf::FullscreenType;
use conf::{WindowMode, WindowSetup};
use crustaceans_of_life::grid::Grid;
use ggez::conf::NumSamples;
use ggez::*;

struct State {
    dt: std::time::Duration,
    grid: Grid,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        self.dt = _ctx.time.delta();
        self.grid.update_state();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::BLACK);
        self.grid.draw(_ctx, &mut canvas)?;
        Ok(())
    }
}
fn main() {
    let mut grid = Grid::new(3, 3);
    let some_initial_config: &[&[u8]] = &[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]];
    let some_other_initial_config: &[&[u8]] = &[&[0, 1, 0], &[0, 1, 0], &[1, 1, 1]];
    grid.try_custom_initial_config(some_initial_config);

    let state = State {
        dt: std::time::Duration::new(0, 0),
        grid: grid,
    };
    let c = conf::Conf::new();
    let mode_for_the_window = WindowMode {
        width: 1000.0,
        height: 500.0,
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
    event::run(ctx, event_loop, state);

    // println!("Lets now try a different config");
    // println!("------------------------------------------------------------- \n");
    // // grid.try_custom_initial_config(some_initial_config);
    // state.grid.print_state();
    // println!("------------------------------------------------------------- \n");
    // state.grid.print_state();
    // println!("------------------------------------------------------------- \n");
    // state.grid.print_state();
    // println!("------------------------------------------------------------- \n");
    // state.grid.print_state();
    // println!("------------------------------------------------------------- \n");
    // state.grid.print_state();
}
