// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    for row in 0..ROWS {
        for col in 0..COLS {
            if grid[row][col] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue; // the cell itself is not its own neighbor
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r < 0 || r >= ROWS as i32 || c < 0 || c >= COLS as i32 {
                continue; // beyond the edge counts as dead
            }
            if grid[r as usize][c as usize] {
                count += 1;
            }
        }
    }
    count
}
// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}
// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(_grid: &Grid) {
    todo!("Implement print_neighbor_counts")
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    todo!("Implement play")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    // TODO (student): write the remaining cases from your truth table before you
    // write next_state. At minimum: a live cell with 4 neighbors, a dead cell with 2.
    #[test]
    fn live_0_dies() {
        assert!(!next_state(true, 0));
    }

    #[test]
    fn live_3_survives() {
        assert!(next_state(true, 3));
    }

    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn live_8_dies() {
        assert!(!next_state(true, 8));
    }

    #[test]
    fn dead_0_stays_dead() {
        assert!(!next_state(false, 0));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }
}
