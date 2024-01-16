use std::fmt::Debug;

use crate::custom_error::AocError;

/// Using a custom bucket queue type instead of BinaryHeap for getting lowest cost node among unvisited.
pub fn solve_bucket(grid: &[u8]) -> usize {
    const MIN_STEPS: isize = 4;
    const MAX_STEPS: isize = 10;

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

            // go in a straight line for up to MAX_STEPS steps
            for step in 1..=MAX_STEPS {
                let next_index = (index as isize + next_dir * step) as usize;
                new_cost += match grid.get(next_index) {
                    Some(b'\n') => break,
                    Some(c) => c - b'0',
                    None => break,
                } as usize;
                
                // only consider adding tile after stepping at least MIN_STEPS times
                if step < MIN_STEPS {
                    continue;
                }
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

pub fn process(input: &str) -> miette::Result<String, AocError> {
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
        assert_eq!("94", process(input)?);
        Ok(())
    }
}
