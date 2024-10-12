pub struct Cell {
    current_state: State,
    previous_state: State,
    position: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

impl Cell {
    pub fn new(current_state: State, previous_state: State, position: (usize, usize)) -> Cell {
        Cell {
            current_state,
            previous_state,
            position,
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
}
