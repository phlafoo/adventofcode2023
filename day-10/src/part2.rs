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
    map[b'|' as usize] = [UP, DOWN];
    map[b'-' as usize] = [LEFT, RIGHT];
    map[b'L' as usize] = [UP, RIGHT];
    map[b'J' as usize] = [UP, LEFT];
    map[b'7' as usize] = [DOWN, LEFT];
    map[b'F' as usize] = [DOWN, RIGHT];
    map[b'S' as usize] = [DOWN, RIGHT];
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

/*
^ 1100
v 0000

< 0100
> 1000
'|'
0000 ^ 1100 = 1100
0000 ^ 0000 = 0000
'-'
0000 ^ 0100 = 0100
0000 ^ 1000 = 1000
'L'
1010 ^ 0010 = 1000
1010 ^ 0100 = 1100
'F'
0100 ^ 0100 = 0000
0100 ^ 1100 = 1000
'J'
0100 ^ 0000 = 0100
0100 ^ 1000 = 1100
'7'
1000 ^ 1000 = 0000
1000 ^ 1100 = 0100

*/

// The idea is to be able to get the next direction by doing a bitwise xor with the current tile
// and previous direction.
mod direction {
    pub const N: u8 = 0b1100;
    pub const S: u8 = 0b0000;
    pub const W: u8 = 0b0100;
    pub const E: u8 = 0b1000;
}

mod tile {
    pub const NS: u8 = 0b0000; // b'|'
    pub const EW: u8 = 0b0000; // b'-'
    pub const NE: u8 = 0b1000; // b'L'
    pub const NW: u8 = 0b0100; // b'J'
    pub const SW: u8 = 0b1000; // b'7'
    pub const SE: u8 = 0b0100; // b'F'
}

const MAP_LEN: usize = 128;
static BIT_MAP: [u8; MAP_LEN] = create_bit_map();

const fn create_bit_map() -> [u8; MAP_LEN] {
    let mut bit_map = [0; MAP_LEN];
    bit_map[b'|' as usize] = tile::NS;
    bit_map[b'-' as usize] = tile::EW;
    bit_map[b'L' as usize] = tile::NE;
    bit_map[b'J' as usize] = tile::NW;
    bit_map[b'7' as usize] = tile::SW;
    bit_map[b'F' as usize] = tile::SE;
    bit_map
}

/// Check east and west. If no match then it must be north and south
fn get_start_direction(input: &[u8], start_index: usize) -> u8 {
    // east
    if let Some(c) = input.get(start_index + 1) {
        if [b'-', b'J', b'7'].contains(c) {
            return direction::E;
        }
    }
    // west
    if let Some(c) = input.get(start_index - 1) {
        if [b'-', b'L', b'F'].contains(c) {
            return direction::W;
        }
    }
    direction::N
}

fn get_next_index(mut index: usize, dir: u8, row_length: usize) -> usize {
    index -= (dir == direction::N) as usize * row_length;
    index += (dir == direction::S) as usize * row_length;
    index -= (dir == direction::W) as usize;
    index += (dir == direction::E) as usize;
    index
}

#[derive(Debug, Clone, Copy)]
struct Vertex {
    x: i32,
    y: i32,
}

impl Vertex {
    pub fn new(x: i32, y: i32) -> Vertex {
        Vertex { x, y }
    }
}

#[derive(Debug, Default)]
struct Perimeter {
    vertices: Vec<Vertex>,
    perimeter_length: usize,
    row_length: usize,
}

impl Perimeter {
    pub fn save_vertex(&mut self, index: usize) {
        self.vertices.push(Vertex::new(
            (index % self.row_length) as i32,
            (index / self.row_length) as i32,
        ))
    }
}

pub fn process_bits(input: &str) -> miette::Result<String, AocError> {
    let mut index = input.find('S').unwrap();

    let mut p = Perimeter {
        row_length: input.find('\n').unwrap() + 1,
        ..Default::default()
    };
    p.save_vertex(index);

    let input = input.as_bytes();

    let mut dir = get_start_direction(input, index);

    // Traverse entire loop and save vertices
    loop {
        p.perimeter_length += 1;
        index = get_next_index(index, dir, p.row_length);

        let c = input[index];
        match c {
            b'S' => break,
            b'L' | b'J' | b'7' | b'F' => p.save_vertex(index),
            _ => (),
        }
        dir ^= BIT_MAP[c as usize]
    }

    // shoelace algorithm
    p.vertices.push(p.vertices[0]);
    let area: i32 = p
        .vertices
        .iter()
        .zip(p.vertices.iter().skip(1))
        .fold(0, |acc, (v1, v2)| acc + (v2.x + v1.x) * (v2.y - v1.y))
        / 2;

    // dbg!(p.perimeter_length, p.vertices, area);
    // Pick's theorem
    let result = i32::abs(area) as usize - p.perimeter_length / 2 + 1;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_0() -> miette::Result<()> {
        let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("1", process(input)?);
        assert_eq!("1", process_bits(input)?);
        Ok(())
    }

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
        assert_eq!("4", process_bits(input1)?);
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
        assert_eq!("4", process_bits(input2)?);
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
        assert_eq!("8", process_bits(input3)?);
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
        assert_eq!("10", process_bits(input4)?);
        Ok(())
    }
}
