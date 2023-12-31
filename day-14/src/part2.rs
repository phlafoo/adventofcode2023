use crate::custom_error::AocError;

use indexmap::IndexSet;
use itertools::Itertools;

struct Platform {
    grid: String,
    width: usize,
    height: usize,
    size: usize, // width * height
}

impl Platform {
    const BALL: u8 = b'O';
    const BLOCK: u8 = b'#';
    const EMPTY: u8 = b'.';

    pub fn new(grid: &str) -> Platform {
        let size = grid.len();
        let width = grid.find('\n').unwrap();
        let height = size / width;
        let grid = grid.to_string();

        Platform {
            grid,
            width,
            height,
            size,
        }
    }

    pub fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn tilt_north(&mut self) {
        // indexing bytes is more performant
        let grid = unsafe { self.grid.as_bytes_mut() };

        // For each column, store the index where the next ball in that column would roll to
        let mut free_slots = (0..=self.width).collect_vec();

        for i in 0..self.size {
            match grid[i] {
                Platform::BLOCK => {
                    // Update the free slot for this column
                    free_slots[i % (self.width + 1)] = i + self.width + 1;
                }
                Platform::BALL => {
                    // Roll ball to free slot and update free slot
                    let free_slot = &mut free_slots[i % (self.width + 1)];
                    grid[i] = Platform::EMPTY;
                    grid[*free_slot] = Platform::BALL;
                    *free_slot += self.width + 1;
                }
                _ => (),
            }
        }
    }

    pub fn tilt_west(&mut self) {
        let grid = unsafe { self.grid.as_bytes_mut() };

        let mut free_slot = 0;
        for i in 0..self.size {
            match grid[i] {
                Platform::BLOCK | b'\n' => {
                    free_slot = i + 1;
                }
                Platform::BALL => {
                    grid[i] = Platform::EMPTY;
                    grid[free_slot] = Platform::BALL;
                    free_slot += 1;
                }
                _ => (),
            }
        }
    }

    pub fn tilt_south(&mut self) {
        let grid = unsafe { self.grid.as_bytes_mut() };

        // Collect indices for the last row of grid
        let mut free_slots = (self.size - self.width..self.size).collect_vec();

        // Need to scan bottom up
        for i in (0..self.size).rev() {
            match grid[i] {
                Platform::BLOCK => {
                    // `saturating_sub` is not technically required but it is more performant
                    free_slots[i % (self.width + 1)] = i.saturating_sub(self.width + 1);
                }
                Platform::BALL => {
                    let free_slot = &mut free_slots[i % (self.width + 1)];
                    grid[i] = Platform::EMPTY;
                    grid[*free_slot] = Platform::BALL;
                    *free_slot = free_slot.saturating_sub(self.width + 1);
                }
                _ => (),
            }
        }
    }

    pub fn tilt_east(&mut self) {
        let grid = unsafe { self.grid.as_bytes_mut() };

        let mut free_slot = self.size - 1;

        // Need to scan right to left
        for i in (0..self.size).rev() {
            match grid[i] {
                Platform::BLOCK | b'\n' => {
                    free_slot = i - 1;
                }
                Platform::BALL => {
                    grid[i] = Platform::EMPTY;
                    grid[free_slot] = Platform::BALL;
                    free_slot -= 1;
                }
                _ => (),
            }
        }
    }

    pub fn calculate_north_load(&self) -> usize {
        let mut result = 0;

        for (row, load) in self
            .grid
            .as_bytes()
            .chunks(self.width + 1)
            .zip((1..=self.height).rev())
        {
            for tile in row.iter() {
                if *tile == Platform::BALL {
                    result += load;
                }
            }
        }
        result
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut platform = Platform::new(input);

    // Using IndexSet to index the set by insertion order
    let mut grid_states: IndexSet<String, ahash::RandomState> = IndexSet::default();

    let total_spin_cycles = 1_000_000_000;

    for i in 0..total_spin_cycles {
        platform.spin_cycle();

        let (first_occurrence, is_new_state) = grid_states.insert_full(platform.grid.clone());

        if !is_new_state {
            let period = i - first_occurrence;
            let final_state_index = first_occurrence + (total_spin_cycles - i - 1) % period;

            platform.grid = grid_states.swap_remove_index(final_state_index).unwrap();

            return Ok(platform.calculate_north_load().to_string());
        }
    }
    // Cycle not found
    Ok(platform.calculate_north_load().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("64", process(input)?);
        Ok(())
    }
}
