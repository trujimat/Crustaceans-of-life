use ggez::*;

const CELL_SIZE: f32 = 30.0;

pub struct Cell {
    current_state: State,
    previous_state: State,
    position: (usize, usize),
    rect_position: (f32, f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

impl Cell {
    pub fn new(current_state: State, previous_state: State, position: (usize, usize)) -> Cell {
        let rect_position = (
            (position.0 as f32 * CELL_SIZE),
            (position.1 as f32 * CELL_SIZE),
        );
        Cell {
            current_state,
            previous_state,
            position,
            rect_position,
        }
    }
    pub fn update_state(&mut self, alive_neighbors: u8) {
        self.previous_state = self.current_state.clone();
        self.current_state = match self.current_state {
            State::Alive if alive_neighbors > 1 && alive_neighbors < 4 => State::Alive,
            State::Alive => State::Dead,
            State::Dead if alive_neighbors == 3 => State::Alive,
            State::Dead => State::Dead,
        };
    }
    pub fn get_previous_state(&self) -> State {
        self.previous_state.clone()
    }
    pub fn get_current_state(&self) -> State {
        self.current_state.clone()
    }
    pub fn set_current_state(&mut self, state: State) {
        self.current_state = state;
    }
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
    ) -> Result<(), GameError> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.rect_position.0,
                self.rect_position.1,
                CELL_SIZE,
                CELL_SIZE,
            ),
            graphics::Color::WHITE,
        )?;

        canvas.draw(&rectangle, graphics::DrawParam::default());
        Ok(())
    }
}
