use crate::constants::DESIRED_FPS;
use crate::grid::Grid;
use ggez::event::MouseButton;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::*;
pub struct GameState {
    dt: std::time::Duration,
    grid: Grid,
    game_started: bool,
    mouse_down: bool,
}

impl GameState {
    pub fn new(rows: usize, cols: usize) -> GameState {
        let grid = Grid::new(rows, cols);

        let game_state = GameState {
            dt: std::time::Duration::new(0, 0),
            grid: grid,
            game_started: false,
            mouse_down: false,
        };

        game_state
    }

    pub fn swap_cell(&mut self, x: f32, y: f32) {
        self.grid.swap_cell(x, y);
    }
}

impl ggez::event::EventHandler<GameError> for GameState {
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
