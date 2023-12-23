use crate::custom_error::AocError;
use phf::phf_map;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

// Associate pipes with directions
// static PIPE_DIR_MAP: phf::Map<u8, [(i32, i32); 2]> = phf_map! {
//     b'|' => [UP,   DOWN],
//     b'-' => [LEFT, RIGHT],
//     b'L' => [UP,   RIGHT],
//     b'J' => [UP,   LEFT],
//     b'7' => [DOWN, LEFT],
//     b'F' => [DOWN, RIGHT],
//     b'S' => [DOWN, RIGHT],
// };
static PIPE_DIR_MAP: [[(i32, i32); 2]; 128] = create_pipe_dir_map();

const fn create_pipe_dir_map() -> [[(i32, i32); 2]; 128] {
    let mut map = [[(0, 0), (0, 0)]; 128];
    map[b'|' as usize] =  [UP,   DOWN];
    map[b'-' as usize] =  [LEFT, RIGHT];
    map[b'L' as usize] =  [UP,   RIGHT];
    map[b'J' as usize] =  [UP,   LEFT];
    map[b'7' as usize] =  [DOWN, LEFT];
    map[b'F' as usize] =  [DOWN, RIGHT];
    map[b'S' as usize] =  [DOWN, RIGHT];
    map
}

/// I probably could have processed the input better to reduce the amount of conditional logic.
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let row_length = (input.find('\n').unwrap() + 1) as i32;
    let start_index = input.find('S').unwrap() as i32;

    // indexing is easier if in bytes
    let input = input.as_bytes();

    // Find first direction to go from start. Guaranteed to have 2 possible directions to go. We
    // just need to find one so we can check 2 directions and if it's not one of them then we select
    // one of the other 2.
    // Assume it's LEFT, then check UP and DOWN.
    let mut start_dir = LEFT;
    let end_dir;
    let mut start_directions = vec![];

    // Check if 'S' is at top or bottom
    if start_index > row_length {
        start_directions.push(UP);
    }
    if start_index < input.len() as i32 - row_length {
        start_directions.push(DOWN);
    }
    for (x, y) in &start_directions {
        let pipe = input[(start_index + row_length * y) as usize];
        if [b'.', b'\n'].contains(&pipe) {
            continue;
        }
        // If the reverse of current direction is one of the pipe's assoc. directions => valid
        // start direction
        if PIPE_DIR_MAP[pipe as usize].contains(&(-x, -y)) {
            start_dir = (*x, *y);
            break;
        }
    }

    // Get starting index and direction
    let mut index = start_index;
    let mut dir = start_dir;

    // Represents a a contiguous horizontal section of loop
    let mut range = index..(index + 1);

    // List of ranges that are part of loop
    let mut loop_ranges = vec![];

    // Find all `ranges` on the loop. Break when back at start
    loop {
        // Move index in direction
        let (x, y) = dir;
        index = index + (row_length * y) + x;

        // Update range based on direction
        match dir {
            UP | DOWN => {
                // start new range
                loop_ranges.push(range);
                range = index..(index + 1);
            }
            LEFT => range.start = index,
            RIGHT => range.end = index + 1,
            _ => panic!("invalid direction"),
        }

        // Get next pipe segment
        let pipe = &input[index as usize];

        if *pipe == b'S' {
            // Back at start, need to update the original range associated with the start tile
            match dir {
                LEFT => {
                    loop_ranges[0].end = range.end;
                }
                RIGHT => {
                    loop_ranges[0].start = range.start;
                }
                _ => (),
            }
            // Save end direction for later use
            end_dir = dir;
            break;
        }
        // get assoc. directions
        let directions = &PIPE_DIR_MAP[*pipe as usize];

        // Select the assoc. direction that is not the reverse of the current direction
        dir = if (-x, -y) == directions[0] {
            directions[1]
        } else {
            directions[0]
        }
    }

    let mut is_inside = false;
    let mut total = 0;
    let mut inside_left_bound = 0;
    let start_replacement = replace_start(start_dir, end_dir);

    // Traverse ranges in order
    loop_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    for range in &loop_ranges {
        // If we were inside, increase total
        if is_inside {
            total += range.start - inside_left_bound;
        }
        // If we have crossed a "boundary", negate `is_inside`
        if match &input[range.start as usize] {
            b'|' => true,
            p => {
                // Need to carefully handle when 'S' is at the beginning or end of range
                let left = if range.start == start_index {
                    start_replacement
                } else {
                    *p
                };
                let right = if range.end - 1 == start_index {
                    start_replacement
                } else {
                    input[range.end as usize - 1]
                };
                crossed_boundary(left, right)
            }
        } {
            is_inside = !is_inside;
        }
        // The current range's right bound is the left bound for the next inside section
        inside_left_bound = range.end;
    }

    Ok(total.to_string())
}

/// Checks if a boundary has been crossed for the following cases:
///     L--7 => true
///     F--7 => true
///     L--J => false
///     F--J => false
#[inline]
fn crossed_boundary(left: u8, right: u8) -> bool {
    if left == b'|' {
        return true;
    }
    PIPE_DIR_MAP[left as usize][0] != PIPE_DIR_MAP[right as usize][0]
}

/// Figure out which corner pipe segment should replace the start tile
#[inline]
fn replace_start(start_dir: (i32, i32), end_dir: (i32, i32)) -> u8 {
    match (start_dir, end_dir) {
        (UP, RIGHT) | (LEFT, DOWN) => b'J',
        (UP, LEFT) | (RIGHT, DOWN) => b'L',
        (DOWN, LEFT) | (RIGHT, UP) => b'F',
        (DOWN, RIGHT) | (LEFT, UP) => b'7',
        _ => b'|',
    }
}

// pub fn process(input: &str) -> miette::Result<String, AocError> {
    
//     Ok("".to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input1 = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!("4", process(input1)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input2 = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!("4", process(input2)?);
        Ok(())
    }

    #[test]
    fn test_process3() -> miette::Result<()> {
        let input3 = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", process(input3)?);
        Ok(())
    }

    #[test]
    fn test_process4() -> miette::Result<()> {
        let input4 = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!("10", process(input4)?);
        Ok(())
    }
}
