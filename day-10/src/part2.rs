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
    b'7' => [DOWN, LEFT],
    b'F' => [DOWN, RIGHT],
    b'S' => [DOWN, RIGHT],
};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let row_length = (input.find('\n').unwrap() + 1) as i32;
    let start_index = input.find('S').unwrap() as i32;

    // indexing is easier if in bytes
    let input = input.as_bytes();

    // dbg!(start_index, row_length, input.len());
    let mut periods = [b'.'; 19878];
    // let mut periods = [b'.'; 107];
    // 219 block

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
        if PIPE_DIR_MAP[&pipe].contains(&(-x, -y)) {
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
            _ => panic!("invalid direction")
        }

        // Get next pipe segment
        let pipe = &input[index as usize];
        
        periods[index as usize] = *pipe;

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
            // dbg!(loop_mask[0].clone());
            // dbg!(loop_mask[loop_mask.len() - 1].clone());
            break;
        }
        // get assoc. directions
        let directions = &PIPE_DIR_MAP[pipe];

        // Select the assoc. direction that is not the reverse of the current direction
        dir = if (-x, -y) == directions[0] {
            directions[1]
        } else {
            directions[0]
        }
    }

    // dbg!(start_dir, end_dir);
    // for (i, c) in periods.iter().enumerate() {
    //     if i as i32 % row_length == 0 {
    //         println!();
    //     }
    //     print!("{}", *c as char);
    // }
    // println!();
    // println!();

    

//    println!("{:?}", loop_mask.clone());

    let mut is_inside = false;
    let mut total = 0;
    let mut inside_left_bound = 0;

    // Traverse ranges in order
    loop_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    for range in &loop_ranges {
        // println!("\x1b[93mError\x1b[0m");
        // let row_start = r.start % row_length;
        // for i in inside_start..r.start {
        //     if i % row_length == 0 {
        //         println!();
        //         continue;
        //     }
        //     if is_inside {
        //         print!("\x1b[101mx\x1b[0m");
        //     } else {
        //         print!(".");
        //     }
        // }
       
        // for i in r.clone() {
            // print!("{} ", input[i as usize] as char);
        // }
        // println!();
        if is_inside {
            total += range.start - inside_left_bound;
        }
        if match &input[range.start as usize] {
            b'|' => true,
            p => {
                    // dbg!(*p as char);
                    // dbg!(input[r.end as usize  - 1] as char);
                    // dbg!(start_dir, end_dir);
                if range.start == start_index {
                    crossed_boundary(replace_start(start_dir, end_dir), input[range.end as usize - 1])
                } else if range.end - 1 == start_index {
                    crossed_boundary(*p, replace_start(start_dir, end_dir))
                } else {
                    crossed_boundary(*p, input[range.end as usize - 1])
                }
                
                // dbg!(*p as char);
                // PIPE_DIR_MAP[p][0] != PIPE_DIR_MAP[&input[r.end as usize]][0]
            }
        } {
            is_inside = !is_inside;
        }
        inside_left_bound = range.end;
        // dbg!(do_switch, is_inside);
        
        // else {
            // for i in inside_start..r.start {
            //     if i % row_length == 0 {
            //         println!();
            //         // continue;
            //     }
            //     print!(".");
            // }
        // }
        
        // for i in r.clone() {
        //     print!("{}", input[i as usize] as char);
        // }
        
        // println!();
        // let row = r.start / row_length;
        // println!("{row}");
    }
    // for i in inside_start..input.len() as i32 {
    //     if i % row_length == 0 {
    //         println!();
    //         // continue;
    //     }
    //     print!(".");
    // }
    // println!();

    // Furthest point will be halfway around loop
    Ok(total.to_string())
}

#[inline]
fn crossed_boundary(left: u8, right: u8) -> bool {
    PIPE_DIR_MAP[&left][0] != PIPE_DIR_MAP[&right][0]
}

#[inline]
fn replace_start(start_dir: (i32, i32), end_dir: (i32, i32)) -> u8 {
    match (start_dir, end_dir) {
        (UP, RIGHT) | (LEFT, DOWN) => b'J',
        (UP, LEFT) | (RIGHT, DOWN) => b'L',
        (DOWN, LEFT) | (RIGHT, UP) => b'F',
        (DOWN, RIGHT) | (LEFT, UP) => b'7',
        _ => panic!(),
    }
}

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
