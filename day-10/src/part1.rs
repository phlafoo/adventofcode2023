use crate::custom_error::AocError;
use phf::phf_map;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

// Associate pipes with directions
static PIPE_DIR_MAP: phf::Map<u8, [(i32, i32); 2]> = phf_map! {
    b'|' => [UP,   DOWN],
    b'-' => [LEFT, RIGHT],
    b'L' => [UP,   RIGHT],
    b'J' => [UP,   LEFT],
    b'7' => [LEFT, DOWN],
    b'F' => [DOWN, RIGHT],
};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let row_length = (input.find('\n').unwrap() + 1) as i32;
    let start_index = input.find('S').unwrap() as i32;
    // indexing is easier if in bytes
    let input = input.as_bytes();

    // Find first direction to go from start
    let mut dir = (0, 0);
    for (x, y) in &[UP, DOWN, LEFT, RIGHT] {
        let pipe = input[(start_index + (row_length * y) + x) as usize];
        if [b'.', b'\n'].contains(&pipe) {
            continue;
        }
        // If the reverse of current direction is one of the pipe's assoc. directions
        if PIPE_DIR_MAP[&pipe].contains(&(-x, -y)) {
            dir = (*x, *y);
            break;
        }
    }
    let mut index = start_index;
    let mut steps = 0;
    loop {
        steps += 1;

        let (x, y) = dir;
        index = index + (row_length * y) + x;
        let pipe = &input[index as usize];

        if *pipe == b'S' {
            break;
        }
        // get assoc. directions
        let directions = &PIPE_DIR_MAP[pipe];

        // Select the direction that is not the reverse of the current direction
        dir = if (-x, -y) == directions[0] {
            directions[1]
        } else {
            directions[0]
        }
    }
    // Furthest point will be halfway around loop
    let result = steps / 2;
    Ok(result.to_string())
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

pub fn process_bits(input: &str) -> miette::Result<String, AocError> {
    let mut index = input.find('S').unwrap();
    let row_length = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let mut dir = get_start_direction(input, index);

    let mut steps = 0;

    // Traverse entire loop and divide total steps by 2 to get result
    loop {
        steps += 1;
        index = get_next_index(index, dir, row_length);

        match input[index] {
            b'S' => break,
            c => dir ^= BIT_MAP[c as usize],
        }
    }
    let result = steps / 2;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input)?);
        assert_eq!("4", process_bits(input)?);
        Ok(())
    }
    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!("8", process(input)?);
        assert_eq!("8", process_bits(input)?);
        Ok(())
    }
}
