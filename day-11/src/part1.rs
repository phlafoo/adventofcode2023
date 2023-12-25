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

/*
The total can be calculated by finding the distance between all galaxy pairs in each dimension
separately and then adding together.

Simple example with 4 galaxies and considering only x dimension:

    t: total distance
    x_i: x coord
    t = |x_0 - x_1| + |x_0 - x_2| + |x_0 - x_3|
        + |x_1 - x_2| + |x_2 - x_3|
        + |x_2 - x_3|

If we sort the coords so that x_i > x_(i+1) then we can drop the absolute values and simplify:
    t = 3x_0 + x_1 - x_2 - 3x_3
Which is a linear operation wrt number of galaxies (instead of quadratic).
In my implementation the galaxy coords are sorted in increasing order so we just reverse the
coefficients with this general formula:
    (1 - g + 2i)
Where g is total number of galaxies.
So we can store and sort the x/y galaxy coords separately.

The expansion can be added by the same method as the naive approach above.

Due to the sort, the time complexity is O(n + klogk) where n is input length and k is number of galaxies
*/
pub fn process_faster(input: &str) -> miette::Result<String, AocError> {
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

    let total_galaxies = galaxies_x.len() as i32;

    // galaxy x-coords need to be sorted
    galaxies_x.sort();

    let x_dist = galaxies_x.iter().enumerate().fold(0, |acc, (i, gx)| {
        acc + (gx + empty_columns[*gx as usize]) * (1 - total_galaxies + (i * 2) as i32)
    });

    // y coords are already sorted
    let y_dist = galaxies_y.iter().enumerate().fold(0, |acc, (i, gy)| {
        acc + (gy + empty_rows[*gy as usize]) * (1 - total_galaxies + (i * 2) as i32)
    });
    let result = x_dist + y_dist;
    
    Ok(result.to_string())
}

/*
Similar approach to `process_faster` but without having to sort the galaxies.
This is done by creating a galaxy mask vector for each dimension.
So if the grid is 10x10 then I would make 2 size 10 vectors where the index represents the
row/column and the value represents the number of galaxies found in the respective row/column.
This way, we can iterate each list and the coords will be in order since they equal the indices.

How to handle having many galaxies sharing a coord? Taking this equation from above:
    t = 3x_0 + x_1 - x_2 - 3x_3
If x_2 == x_3 then the value at the corresponding column index would be 2. We can factor like so:
    t = 3x_0 + x_1 - 4x_2

I came up with this formula to calculate the coefficient for a give row/column:
    c = n(n + 2g - t)
Where
    n = number of galaxies in this row/column (value from vector)
    g = number of galaxies that have been considered already
    t = total galaxies

Expansion is also handled when iterating the galaxy coord vectors by incrementing an expansion
counter whenever a zero is encountered.

Time complexity is O(n) where n is input length
*/
pub fn process_fastest(input: &str) -> miette::Result<String, AocError> {
    // dim => dimension aka side length (assume square grid).
    // padding is the number of characters used for new line
    //   with windows it is 1 ("\n") (my test input)
    //   with linux it is 2 ("\r\n") (actual input)
    let (dim, padding) = match input.find('\r') {
        Some(i) => (i, 2),
        None => (input.find('\n').unwrap(), 1)
    };
    // dbg!(input.len(), dim, padding);

    let input = input.as_bytes();

    let mut galaxies_x = vec![0; dim];
    let mut galaxies_y = vec![0; dim];
    let mut total_galaxies = 0;

    for (index, c) in input.iter().enumerate() {
        if *c == b'#' {
            total_galaxies += 1;
            let v = Vertex::from_index(index, dim + padding);
            galaxies_x[v.x as usize] += 1;
            galaxies_y[v.y as usize] += 1;
        }
    }

    let x_dist = sum_diff_all_pairs(&galaxies_x, total_galaxies);
    let y_dist = sum_diff_all_pairs(&galaxies_y, total_galaxies);
    let result = x_dist + y_dist;

    Ok(result.to_string())
}

fn sum_diff_all_pairs(galaxy_coords: &[usize], total_galaxies: usize) -> i128 {
    let mut expansion = 0;
    let mut curr_galaxy = 0;

    galaxy_coords.iter().enumerate().fold(0, |acc, (i, &c)| {
        let f = c as i128 * (c as i128 + 2 * curr_galaxy - total_galaxies as i128);

        curr_galaxy += c as i128;
        expansion += (c == 0) as i128;
        
        acc + f * (i as i128 + expansion)
    })
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
        assert_eq!("374", process_fastest(input)?);
        Ok(())
    }
}
/*
cols: [2, 1, 0, 1, 1, 0, 1, 2, 0, 1]
rows: [1, 1, 1, 0, 1, 1, 1, 0, 1, 2]
t=9 (total)
c_i, g_i, e
for c in cols:
    c

t = 4
c=1, g=0: f = -3
c=2, g=0: f = -4
c=3, g=0: f = -3
c=4, g=0: f = 0

t = 9
c=1, g=0: f =  -8 -8
c=2, g=0: f = -14 -6
c=3, g=0: f = -18 -4
c=4, g=0: f = -20 -2
c=5, g=0: f = -20  0
c=6, g=0: f = -18  2
c=7, g=0: f = -14  4
c=8, g=0: f =  -8  6
c=9, g=0: f =   0  8
c(c-t)

cols: [1, 2, 0, 5*, 1, 0]
g_i = 3
c_i = 3
t = 9
f = 10

c=3, g=0: f = -18
c=3, g=1: f = -10
c=3, g=2: f =  -4
c(c-t) - (g)(g-t)
(c+g + 2)(c+g+2-t) - g(g-t)
*** (2 + c)(2 + c + 2g - t)
indexc cols with c_i, but use c_i + expansion for actual calculation?


c(c-t) - (g-1)(g-1-t) ???



acc + (gx * (1 - galaxy_count + (i * 2) as i32)));
1 - t + ((c - 1) * 2)

*/