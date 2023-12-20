
// use std::collections::HashMap;

use crate::custom_error::AocError;
use phf::phf_map;

// #[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// struct Dir {
//     x: i32,
//     y: i32,
// }

// impl Dir {
//     const UP   : Dir = Dir { x:  0, y: -1 };
//     const DOWN : Dir = Dir { x:  0, y:  1 };
//     const LEFT : Dir = Dir { x: -1, y:  0 };
//     const RIGHT: Dir = Dir { x:  1, y:  0 };
// }


// impl Direction {
//     pub fn rev(&self) -> Direction {
//         match self {
//             Direction::Up => Direction::Down,
//             Direction::Down => Direction::Up,
//             Direction::Left => Direction::Right,
//             Direction::Right => Direction::Left,
//         }
//     }

    // pub fn get_index(&self, row_length: i32) -> usize {
    //     match dir {
    //         Direction::Up => index.saturating_sub(row_length),
    //         Direction::Down => index + row_length,
    //         Direction::Left => index.saturating_sub(1),
    //         Direction::Right => index + 1,
    //     };
    // }
    // }

const UP   : (i32, i32) = ( 0, -1);
const DOWN : (i32, i32) = ( 0,  1);
const LEFT : (i32, i32) = (-1,  0);
const RIGHT: (i32, i32) = ( 1,  0);

static PIPE_DIR_MAP: phf::Map<u8, [(i32, i32); 2]> = phf_map! {
    b'|' => [UP,   DOWN],
    b'-' => [LEFT, RIGHT],
    b'L' => [UP,   RIGHT],
    b'J' => [UP,   LEFT],
    b'7' => [LEFT, DOWN],
    b'F' => [DOWN, RIGHT],
};
// static PIPE_DIR_MAP: phf::Map<u8, [Direction; 2]> = phf_map! {
//     b'|' => [Direction::Up, Direction::Down],
//     b'-' => [Direction::Left, Direction::Right],
//     b'L' => [Direction::Up, Direction::Right],
//     b'J' => [Direction::Up, Direction::Left],
//     b'7' => [Direction::Left, Direction::Down],
//     b'F' => [Direction::Down, Direction::Right],
// };
// static DIR_INDEX_MAP: phf::Map<Direction, (i32, i32)> = phf_map! {
//     Direction::Up => (0, -1),
//     Direction::Up => (0, -1),
// };

#[allow(arithmetic_overflow)]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let row_length = (input.find('\n').unwrap() + 1) as i32;
    let start_index = input.find('S').unwrap() as i32;
    let input = input.as_bytes();

    let mut dir = (0, 0);
    for (x, y) in &[UP, DOWN, LEFT, RIGHT] {
        let p = input[(start_index + (row_length * y) + x) as usize];
        if [b'.', b'\n'].contains(&p) {
            continue;
        }
        if PIPE_DIR_MAP[&p].contains(&(-x, -y)) {
            dir = (*x, *y);
            break;
        }
    };
    let mut index = start_index;
    let mut steps = 0;
    loop {
        let (x, y) = dir;
        index = index + (row_length * y) + x;
        steps += 1;
        let p = &input[index as usize];
        if *p == b'S' {
            break;
        }
        let dirs = &PIPE_DIR_MAP[p];
        
        dir = if (-x, -y) == dirs[0] {
            dirs[1]
        } else {
            dirs[0]
        }
    }
    // dbg!(steps);
    let result = steps / 2;
    Ok(result.to_string())
}
// #[allow(arithmetic_overflow)]
// pub fn process(
//     input: &str,
// ) -> miette::Result<String, AocError> {
//     let row_length = (input.find('\n').unwrap() + 1) as i32;
//     let start_index = input.find('S').unwrap() as i32;
//     let input = input.as_bytes();

//     let dir_index_map: HashMap<Direction, (i32, i32)> = HashMap::from([
//         (Direction::Up, (0, -1)),
//         (Direction::Down, (0, 1)),
//         (Direction::Left, (-1, 0)),
//         (Direction::Right, (1, 0)),
//     ]);
//     // let neighbors: [(i32, i32); 4] = [
//     //     (0, -1),
//     //     (0, 1),
//     //     (-1, 0),
//     //     (1, 0),
//     // ];
//     // println!("{}", input[start_index - row_length] as char);
//     // println!("{}", input[start_index] as char);
//     // println!("{}", input[start_index + row_length] as char);
//     let mut dir = &Direction::Up;
//     for (d, (x, y)) in &dir_index_map {
//         let p = input[(start_index + (row_length * y) + x) as usize];
//         if [b'.', b'\n'].contains(&p) {
//             continue;
//         }
//         if PIPE_DIR_MAP[&p].contains(&d.rev()) {
//             dir = d;
//             break;
//         }
//     };
//     let mut index = start_index;
//     let mut steps = 0;
//     loop {
//         let (x, y) = dir_index_map[dir];
//         index = index + (row_length * y) + x;
//         steps += 1;
//         let p = &input[index as usize];
//         if *p == b'S' {
//             break;
//         }
//         let dirs = &PIPE_DIR_MAP[p];
        
//         dir = if dir.rev() == dirs[0] {
//             &dirs[1]
//         } else {
//             &dirs[0]
//         }
//     }
//     // dbg!(steps);
//     let result = steps / 2;
//     Ok(result.to_string())
// }

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
