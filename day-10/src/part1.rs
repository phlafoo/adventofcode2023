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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input1 = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let input2 = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!("4", process(input1)?);
        assert_eq!("8", process(input2)?);
        Ok(())
    }
}
