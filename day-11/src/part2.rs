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

    let mut empty_rows = vec![999_999; column_length];
    let mut empty_columns = vec![999_999; row_length];
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

// See part 1 for explanation
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
        expansion += (c == 0) as i128 * 999_999;
        
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
        // assert_eq!("82000210", process(input)?);
        assert_eq!("82000210", process_fastest(input)?);
        Ok(())
    }
}
