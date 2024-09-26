pub mod cell {
    pub struct Cell {
        state: State,
        position: (i32, i32),
    }

    enum State {
        Alive,
        Dead,
    }

    impl Cell {
        fn update_state(mut self) {}
    }
}
