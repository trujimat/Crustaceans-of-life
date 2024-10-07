pub mod cell {
    pub struct Cell {
        pub current_state: State,
        pub previous_state: State,
        pub position: (i32, i32),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        Alive,
        Dead,
    }

    impl Cell {
        pub fn update_state(&mut self, neighbors: &[Cell]) {
            self.previous_state = self.current_state.clone();

            let mut alive_neighbors = 0;
            for neighbor in neighbors.into_iter() {
                if neighbor.current_state == State::Alive {
                    alive_neighbors += 1;
                }
            }

            self.current_state = match self.current_state {
                State::Alive if alive_neighbors > 1 && alive_neighbors < 4 => State::Alive,
                State::Alive => State::Dead,
                State::Dead if alive_neighbors == 3 => State::Alive,
                State::Dead => State::Dead,
            };
        }
        fn get_previous_state(self) {}
        fn get_neighbor_state(self) {}
    }
}
