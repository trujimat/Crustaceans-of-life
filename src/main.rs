use crustaceans_of_life::grid::{self, Grid};
fn main() {
    let mut grid = Grid::new(3, 3);
    grid.update_state();
    grid.print_state();
    println!("------------------------------------------------------------- \n");
    grid.update_state();
    grid.print_state()
}
