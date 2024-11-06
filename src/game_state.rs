use crate::constants::DESIRED_FPS;
use crate::grid::Grid;
use ggez::*;
pub struct GameState {
    dt: std::time::Duration,
    grid: Grid,
}

impl GameState {
    pub fn new(rows: usize, cols: usize, config: &[&[u8]]) -> GameState {
        let mut grid = Grid::new(rows, cols);
        grid.try_custom_initial_config(config);

        let game_state = GameState {
            dt: std::time::Duration::new(0, 0),
            grid: grid,
        };

        game_state
    }
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        while _ctx.time.check_update_time(DESIRED_FPS) {
            self.dt = _ctx.time.delta();
            self.grid.update_state();
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
