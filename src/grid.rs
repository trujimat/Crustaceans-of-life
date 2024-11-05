use crate::cell::{Cell, State};
use ggez::*;

pub struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        let mut cells = Vec::new();
        for x in 0..rows + 1 {
            let mut row = Vec::new();
            for y in 0..cols + 1 {
                row.push(Cell::new(State::Dead, State::Dead, (x, y)));
            }
            cells.push(row);
        }
        Grid { rows, cols, cells }
    }

    pub fn update_state(&mut self) {
        for x in 0..self.rows {
            for y in 0..self.cols {
                self.update_cell_state(x, y);
            }
        }
        println!("------------------------------------------------------------- \n");
        self.print_state();
    }

    pub fn update_cell_state(&mut self, x: usize, y: usize) {
        let dx = self.get_dx(x);
        let dy = self.get_dy(y);
        let alive_neighbors = self.get_alive_neighbors(x, y, dx, dy);

        self.cells[x][y].update_state(alive_neighbors);
    }

    fn get_dx(&self, x: usize) -> (i8, i8) {
        let mut dx = (-1, 1);
        if x == 0 {
            dx = (0, 1); // No left neighbor
        } else if x == self.rows - 1 {
            dx = (-1, 0); // No right neighbor
        }
        dx
    }

    fn get_dy(&self, y: usize) -> (i8, i8) {
        let mut dy = (-1, 1);
        if y == 0 {
            dy = (0, 1); // No left neighbor
        } else if y == self.cols - 1 {
            dy = (-1, 0); // No right neighbor
        }
        dy
    }

    fn get_alive_neighbors(&self, x: usize, y: usize, dx: (i8, i8), dy: (i8, i8)) -> u8 {
        let mut alive_neighbors = 0;
        for x_offset in dx.0..dx.1 + 1 {
            for y_offset in dy.0..dy.1 + 1 {
                let x_index = x as i8 + x_offset;
                let y_index = y as i8 + y_offset;
                if x < x_index as usize || (x == x_index as usize && y < y_index as usize) {
                    if (x_index as usize != x || y_index as usize != y)
                        && self.cells[x_index as usize][y_index as usize].get_current_state()
                            == State::Alive
                    {
                        alive_neighbors += 1;
                        // println!(
                        //     "at least one alive neighbor, alive_neigbors: {}",
                        //     alive_neighbors
                        // );
                        // println!("and the cell is {:?}", (x, y));
                    }
                } else {
                    if (x_index as usize != x || y_index as usize != y)
                        && self.cells[x_index as usize][y_index as usize].get_previous_state()
                            == State::Alive
                    {
                        alive_neighbors += 1;
                        // println!(
                        //     "at least one alive neighbor 2, alive_neigbors: {}",
                        //     alive_neighbors
                        // );
                        // println!("and the cell is {:?}", (x, y));
                    }
                }
            }
        }
        alive_neighbors
    }

    pub fn print_state(&self) {
        for x in 0..self.rows {
            for y in 0..self.cols {
                println!(
                    "The state of the observed cell {}{} is {:?} \n",
                    x,
                    y,
                    self.cells[x][y].get_current_state()
                );
            }
        }
    }

    pub fn try_custom_initial_config(&mut self, config: &[&[u8]]) {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if config[x][y] == 1 {
                    self.cells[x][y].set_current_state(State::Alive);
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
    ) -> Result<(), GameError> {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.cells[x][y].get_current_state() == State::Alive {
                    self.cells[x][y].draw(ctx, canvas)?;
                }
            }
        }
        Ok(())
    }
}
