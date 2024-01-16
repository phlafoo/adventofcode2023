use std::{collections::BinaryHeap, fmt::Debug};

use crate::custom_error::AocError;

/// Dijkstra's algo with some adjustments
pub fn solve_2dim(grid: &[&[u8]]) -> i32 {
    // stores the best cost for each alignmnet (up/down and left/right) for each grid position
    let mut cost_cache = vec![vec![[i32::MIN; 2]; grid[0].len()]; grid.len()];

    // This is a max heap but we want min cost, so we negate all costs until we return the result
    let mut unvisited = BinaryHeap::new();
    // (cost, (row, column), (direction_row, direction_column))
    unvisited.push((0, (0_isize, 0_isize), (0_isize, 0_isize)));

    while let Some((cost, (row, col), dir)) = unvisited.pop() {
        // Check if reached end
        if row as usize == grid.len() - 1 && col as usize == grid[0].len() - 1 {
            return -cost;
        }
        // Check if there is already a better cost for this tile and alignment
        if cost < cost_cache[row as usize][col as usize][(dir.0 == 0) as usize] {
            continue;
        }
        for (next_dir_y, next_dir_x) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            // We only turn left and right, so skip if next alignment is same as current
            if dir == (next_dir_y, next_dir_x) || dir == (-next_dir_y, -next_dir_x) {
                continue;
            }
            let mut new_cost = cost;
            // go in a straight line for up to 3 steps
            for step in 1..=3 {
                let next_row = (row + next_dir_y * step) as usize;
                let next_col = (col + next_dir_x * step) as usize;
                if next_row >= grid.len() || next_col >= grid[0].len() {
                    break;
                }
                new_cost -= (grid[next_row][next_col] - b'0') as i32;
                let best = &mut cost_cache[next_row][next_col][(next_dir_y == 0) as usize];
                if new_cost > *best {
                    // if the new cost is better, we update the cache and add this tile as unvisited
                    *best = new_cost;
                    unvisited.push((
                        new_cost,
                        (next_row as isize, next_col as isize),
                        (next_dir_y, next_dir_x),
                    ));
                }
            }
        }
    }
    unreachable!();
}

/// Same as `solve_2dim`` but indexing 1dim byte array instead of 2dim. This is a bit more performant.
pub fn solve_1dim(grid: &[u8]) -> i32 {
    let width = grid.iter().position(|&c| c == b'\n').unwrap() as isize + 1;
    let grid_len = grid.len();

    // stores the best cost for each alignmnet (up/down and left/right) for each grid position
    let mut cost_cache = vec![[i32::MIN; 2]; grid_len];

    // This is a max heap but we want min cost, so we negate all costs until we return the result
    let mut unvisited = BinaryHeap::new();

    // (cost, index, direction)
    unvisited.push((0, 0, 0_isize));

    while let Some((cost, index, dir)) = unvisited.pop() {
        // Check if reached end
        if index == grid_len - 1 {
            return -cost;
        }
        // Check if there is already a better cost for this tile and alignment
        if cost < cost_cache[index][(dir.abs() == 1) as usize] {
            continue;
        }
        // Determine which directions to try next. We only want to go left and right (except from the start tile)
        let next_dirs = match dir.abs() {
            1 => [-width, width],
            d if d == width => [-1, 1],
            0 => [1, width],
            _ => unreachable!(),
        };
        for next_dir in next_dirs {
            let mut new_cost = cost;

            // go in a straight line for up to 3 steps
            for step in 1..=3 {
                let next_index = (index as isize + next_dir * step) as usize;
                if next_index as isize % width == width - 1 || next_index >= grid_len {
                    break;
                }
                new_cost -= (grid[next_index] - b'0') as i32;
                let best = &mut cost_cache[next_index][(next_dir.abs() == 1) as usize];
                if new_cost > *best {
                    // if the new cost is better, we update the cache and add this tile as unvisited
                    *best = new_cost;
                    unvisited.push((new_cost, next_index, next_dir));
                }
            }
        }
    }
    unreachable!();
}

/// Using a custom bucket queue type instead of BinaryHeap for getting lowest cost node among unvisited.
pub fn solve_bucket(grid: &[u8]) -> usize {
    let width = grid.iter().position(|&c| c == b'\n').unwrap() as isize + 1;
    let grid_len = grid.len();

    // stores the best cost for each alignmnet (up/down and left/right) for each grid position
    let mut cost_cache = vec![[usize::MAX; 2]; grid_len];

    let mut unvisited = BucketQueue::new();

    // cost, (index, direction)
    unvisited.push(0, (0, 0_isize));

    while let Some((cost, (index, dir))) = unvisited.pop() {
        // Check if reached end
        if index == grid_len - 1 {
            return cost;
        }
        // Check if there is already a better cost for this tile and alignment
        if cost > cost_cache[index][(dir.abs() == 1) as usize] {
            continue;
        }
        // Determine which directions to try next. We only want to go left and right (except from the start tile)
        let next_dirs = match dir.abs() {
            1 => [-width, width],
            d if d == width => [-1, 1],
            0 => [1, width],
            _ => unreachable!(),
        };
        for next_dir in next_dirs {
            let mut new_cost = cost;

            // go in a straight line for up to 3 steps
            for step in 1..=3 {
                let next_index = (index as isize + next_dir * step) as usize;
                new_cost += match grid.get(next_index) {
                    Some(b'\n') => break,
                    Some(c) => c - b'0',
                    None => break,
                } as usize;
                let best = &mut cost_cache[next_index][(next_dir.abs() == 1) as usize];
                if new_cost < *best {
                    // if the new cost is better, we update the cache and add this tile as unvisited
                    *best = new_cost;
                    unvisited.push(new_cost, (next_index, next_dir));
                }
            }
        }
    }
    unreachable!();
}

// Copied from https://github.com/kcaffrey/aoc2023/blob/main/src/bin/17.rs
#[derive(Default, Clone)]
pub struct BucketQueue<T> {
    buckets: Vec<Vec<T>>,
    first_non_empty: Option<usize>,
}

impl<T: Debug + Copy> BucketQueue<T> {
    pub fn new() -> Self {
        Self {
            buckets: vec![],
            first_non_empty: None,
        }
    }
    pub fn push(&mut self, cost: usize, value: T) {
        if cost + 1 > self.buckets.len() {
            self.buckets
                .resize_with(cost + 1, || Vec::with_capacity(64));
        }
        self.buckets[cost].push(value);
        if self.first_non_empty.filter(|&f| cost >= f).is_none() {
            self.first_non_empty = Some(cost);
        }
    }
    pub fn pop(&mut self) -> Option<(usize, T)> {
        let Some(cost) = self.first_non_empty else {
            return None;
        };
        let value = self.buckets[cost].pop().unwrap();
        if self.buckets[cost].is_empty() {
            self.first_non_empty =
                (cost + 1..self.buckets.len()).find(|&i| !self.buckets[i].is_empty())
        }
        Some((cost, value))
    }
}

pub fn process_heap(input: &str) -> miette::Result<String, AocError> {
    let result = solve_1dim(input.as_bytes());
    Ok(result.to_string())
}

pub fn process_bucket(input: &str) -> miette::Result<String, AocError> {
    let result = solve_bucket(input.as_bytes());
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("102", process_heap(input)?);
        assert_eq!("102", process_bucket(input)?);
        Ok(())
    }
}
