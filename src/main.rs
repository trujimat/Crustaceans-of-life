use crustaceans_of_life::cell::Cell;
use crustaceans_of_life::cell::State::Alive;
use crustaceans_of_life::cell::State::Dead;
fn main() {
    let mut tha_cell = Cell {
        current_state: Dead,
        previous_state: Dead,
        position: (2, 2),
    };
    let mut _grid = [
        Cell {
            current_state: Alive,
            previous_state: Dead,
            position: (1, 1),
        },
        Cell {
            current_state: Dead,
            previous_state: Dead,
            position: (2, 1),
        },
        Cell {
            current_state: Alive,
            previous_state: Dead,
            position: (3, 1),
        },
        Cell {
            current_state: Dead,
            previous_state: Dead,
            position: (1, 2),
        },
        Cell {
            current_state: Alive,
            previous_state: Dead,
            position: (3, 2),
        },
        Cell {
            current_state: Alive,
            previous_state: Dead,
            position: (1, 3),
        },
        Cell {
            current_state: Dead,
            previous_state: Dead,
            position: (2, 3),
        },
        Cell {
            current_state: Dead,
            previous_state: Dead,
            position: (3, 3),
        },
    ];

    // Lets iterate over just 2 frames
    tha_cell.update_state(&_grid);
    println!(
        "The state of the observed cell is {:?} \n",
        tha_cell.current_state
    );

    tha_cell.update_state(&_grid);
    println!(
        "The state of the observed cell is {:?} \n",
        tha_cell.current_state
    );
}
