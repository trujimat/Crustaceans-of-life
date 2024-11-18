use crate::constants::{CELL_SIZE, DESIRED_FPS, MARGIN};
use crate::grid::Grid;
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::MouseButton;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::*;
use ggez::{event, ContextBuilder, GameError};

pub struct GameState {
    dt: std::time::Duration,
    grid: Grid,
    game_started: bool,
    mouse_down: bool,
    rows: usize,
    cols: usize,
}

impl GameState {
    /// Creates a new GameState instance
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = Grid::new(rows, cols);
        Self {
            dt: std::time::Duration::new(0, 0),
            grid,
            game_started: false,
            mouse_down: false,
            rows,
            cols,
        }
    }

    /// Sets up the window and returns the ggez context and event loop
    pub fn setup_window(&self) -> (Context, event::EventLoop<()>) {
        let width = ((2.0 * MARGIN) + (self.cols as f32 * CELL_SIZE)) / 2.0;
        let height = ((2.0 * MARGIN) + (self.rows as f32 * CELL_SIZE)) / 2.0;

        let mode_for_the_window = WindowMode {
            width: width,
            height: height,
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
            title: "The Game of Life".to_owned(),
            samples: NumSamples::One,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        };

        let mut conf = conf::Conf::new();
        conf.window_mode = mode_for_the_window;
        conf.window_setup = window_set_up;

        ContextBuilder::new("crustaceans_of_life", "trujimat")
            .default_conf(conf)
            .build()
            .unwrap()
    }

    /// Swaps the state of a cell at a given position
    pub fn swap_cell(&mut self, x: f32, y: f32) {
        self.grid.swap_cell(x, y);
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        if self.game_started {
            while _ctx.time.check_update_time(DESIRED_FPS) {
                self.dt = _ctx.time.delta();
                self.grid.update_state();
            }
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_down = true;
        println!("Mouse button pressed: {button:?}, x: {x}, y: {y}");
        if !self.game_started {
            self.swap_cell(x, y);
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, repeat: bool) -> GameResult {
        println!(
            "Key pressed: scancode {}, keycode {:?}, modifier {:?}, repeat: {}",
            input.scancode, input.keycode, input.mods, repeat
        );
        if input.keycode == Some(KeyCode::Space) && !self.game_started {
            println!("Game Started!");
            self.game_started = true;
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::BLACK);
        self.grid.draw(_ctx, &mut canvas)?;
        canvas.finish(_ctx)?;
        Ok(())
    }
}
