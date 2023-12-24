use crate::custom_error::AocError;

#[derive(Debug, Default, Clone, Copy)]
struct Vertex {
    x: i64,
    y: i64,
}

impl Vertex {
    pub fn from_index(index: usize, row_length: usize) -> Vertex {
        Vertex {
            x: (index % row_length) as i64,
            y: (index / row_length) as i64,
        }
    }
    pub fn manhattan_dist(&self, other: &Vertex) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }
}

// Brute force O(n^2) approach
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let row_length = input.find('\n').unwrap() + 1;
    let column_length = input.len() / row_length + 1;

    let input = input.as_bytes();

    let mut empty_rows = vec![1; column_length];
    let mut empty_columns = vec![1; row_length];
    let mut galaxies = vec![];

    for (index, tile) in input.iter().enumerate() {
        if *tile == b'#' {
            let v = Vertex::from_index(index, row_length);
            empty_rows[v.y as usize] = 0;
            empty_columns[v.x as usize] = 0;
            galaxies.push(v);
        }
    }
    // Find how much to add to corresponding rows/columns.
    // e.g.
    //  0 1 0 0 1 0
    // becomes
    //  0 1 1 1 2 2
    let mut acc = 0;
    for r in &mut empty_rows {
        acc += *r;
        *r = acc;
    }
    acc = 0;
    for c in &mut empty_columns {
        acc += *c;
        *c = acc;
    }

    // Adjust coords for galaxies based on empty rows/columns
    for g in &mut galaxies {
        g.x += empty_columns[g.x as usize];
        g.y += empty_rows[g.y as usize];
    }

    // Sum manhattan distance between all galaxy pairs
    let result = galaxies.iter().enumerate().fold(0, |acc, (index, g1)| {
        acc + galaxies[(index + 1)..]
            .iter()
            .fold(0, |acc_inner, g2| acc_inner + g1.manhattan_dist(g2))
    });

    Ok(result.to_string())
}


pub fn process_linear(input: &str) -> miette::Result<String, AocError> {
    let row_length = input.find('\n').unwrap() + 1;
    let column_length = input.len() / row_length + 1;

    let input = input.as_bytes();

    let mut empty_rows = vec![1; column_length];
    let mut empty_columns = vec![1; row_length];
    let mut galaxies_x = vec![];
    let mut galaxies_y = vec![];

    for (index, tile) in input.iter().enumerate() {
        if *tile == b'#' {
            let x = index % row_length;
            let y = index / row_length;
            empty_rows[y] = 0;
            empty_columns[x] = 0;
            galaxies_x.push(x as i32); // this won't be in order
            galaxies_y.push(y as i32); // this will be in order
        }
    }
    // Find how much to add to corresponding rows/columns.
    // e.g.
    //  0 1 0 0 1 0
    // becomes
    //  0 1 1 1 2 2
    let mut acc = 0;
    for r in &mut empty_rows {
        acc += *r;
        *r = acc;
    }
    acc = 0;
    for c in &mut empty_columns {
        acc += *c;
        *c = acc;
    }

    // galaxies_x.sort();
    let gx_len = galaxies_x.len() as i32;

    let x_dist = galaxies_x.iter().enumerate().fold(0, |acc, (i, gx)| {
        acc + (gx + empty_columns[*gx as usize]) * (1 - gx_len + (i * 2) as i32)
    });

    let gy_len = galaxies_y.len() as i32;

    let y_dist = galaxies_y.iter().enumerate().fold(0, |acc, (i, gy)| {
        acc + (gy + empty_rows[*gy as usize]) * (1 - gy_len + (i * 2) as i32)
    });
    let result = x_dist + y_dist;
    

    // // Adjust coords for galaxies based on empty rows/columns
    // for g in &mut galaxies {
    //     g.x += empty_columns[g.x as usize];
    //     g.y += empty_rows[g.y as usize];
    // }
    // galaxies.sort_by(|g1, g2| g2.x.cmp(&g1.x));

    // let g_len = galaxies.len();
    // dbg!(g_len);
    // let x_sum = galaxies.iter().enumerate().fold(0, |acc, (index, g)| {
    //     let multiplier = g_len as i64 - 1 - (2 * index) as i64;
    //     // dbg!(index, multiplier);
    //     acc + g.x * multiplier
    // });
    // dbg!(x_sum);

    // galaxies.sort_by(|g1, g2| g2.y.cmp(&g1.y));
    // // dbg!(galaxies.clone());
    // let y_sum = galaxies.iter().enumerate().fold(0, |acc, (index, g)| {
    //     let multiplier = g_len as i64 - 1 - (2 * index) as i64;
    //     // dbg!(index, multiplier);
    //     acc + g.y * multiplier
    // });
    // dbg!(y_sum);
    
    // Sum manhattan distance between all galaxy pairs
    // let result = galaxies.iter().enumerate().fold(0, |acc, (index, g1)| {
    //     acc + galaxies[(index + 1)..]
    //         .iter()
    //         .fold(0, |acc_inner, g2| acc_inner + g1.manhattan_dist(g2))
    // });
    // let result = x_sum + y_sum;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process_linear(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "\
.#..#.
..#.#.";
/*
..#...#..
...#..#..

x = 2, 6, 3, 6 (sum = 17)
x = 2 * 4 + 6 * 3 + 3 * 2 + 6 * 1
x = 8 + 18 + 6 + 6
x = 38

x sorted: 6, 6, 3, 2
ans = 3*6 + 6 - 3 - 3*2
ans = 18 + 3 - 6 = 15


ans = |1 - 6|*(17 - 6)
        + |1 - 3|*(11 - 3)
        + |1 - 6|*(8 - 6)
ans = 5*11

x = 4 * 3 + 1 * 2 + 4 * 1
x = 18 - 3

x = 5 * 3 + 2 * 2 + 5 * 1
x = 24 - 6

x = 6 * 3 + 3 * 2 + 6 * 1
x = 30 - 9


x = 3 * 3 + 0 * 2 + 3 * 1
x = 12



x = |2 - 6| + |2 - 3| + |2 - 6|
    + |6 - 3| + |6 - 6|
    + |3 - 6|
x = 4 + 1 + 4
    + 3 + 0
    + 3
x = 15

y = 1, 1, 2, 2

y = |0 - 0| + |0 - 1| + |0 - 1|
    + |0 - 1| + |0 - 1|
    + |1 - 1|
y = 0 + 1 + 1
    + 1 + 1
    + 0
y = 4

x + y = 19
*/
        assert_eq!("19", process_linear(input)?);
        Ok(())
    }
}
