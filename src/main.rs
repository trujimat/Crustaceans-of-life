use crustaceans_of_life::grid::Grid;
fn main() {
    let mut grid = Grid::new(3, 3);
    let some_initial_config: &[&[u8]] = &[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]];
    let some_other_initial_config: &[&[u8]] = &[&[0, 1, 0], &[0, 1, 0], &[1, 1, 1]];

    println!("Lets now try a different config");
    println!("------------------------------------------------------------- \n");
    grid.try_custom_initial_config(some_other_initial_config);
    grid.print_state();
    println!("------------------------------------------------------------- \n");
    grid.update_state();
    grid.print_state();
    println!("------------------------------------------------------------- \n");
    grid.update_state();
    grid.print_state();
    println!("------------------------------------------------------------- \n");
    grid.update_state();
    grid.print_state();
    println!("------------------------------------------------------------- \n");
    grid.update_state();
    grid.print_state();
}
