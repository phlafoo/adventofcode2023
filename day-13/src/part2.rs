use crate::custom_error::AocError;

// use 2 for CRLF, 1 for LF
const PADDING: usize = 2;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let patterns = input.split("\r\n\r\n"); // use "\n\n" for LF
    let mut result = 0;

    'patterns: for pattern in patterns {
        // check for reflection at row
        let row_length = pattern.lines().next().unwrap().len();
        for row in 0..pattern.len() / (row_length + PADDING) {
            if is_reflected_at_row(pattern, row, row_length) {
                result += (row + 1) * 100;
                continue 'patterns;
            }
        }
        // check for reflection at column
        for col in 0..row_length {
            if is_reflected_at_column(pattern, col) {
                result += col + 1;
                continue 'patterns;
            }
        }
        panic!("No reflection found for pattern:\n{pattern}");
    }
    Ok(result.to_string())
}

fn is_reflected_at_column(pattern: &str, col: usize) -> bool {
    // there must be exactly one character different in the reflection to return true
    let mut one_diff = false;

    // check for reflection around column line by line, moving outwards from reflection column
    for line in pattern.lines() {
        let line = line.as_bytes();
        for (i, j) in (0..=col).rev().zip(col + 1..line.len()) {
            if line[i] != line[j] {
                if one_diff {
                    return false;
                }
                one_diff = true;
            }
        }
    }
    one_diff
}

fn is_reflected_at_row(pattern: &str, row: usize, row_length: usize) -> bool {
    // there must be exactly one character different in the reflection to return true
    let mut one_diff = false;

    // check for reflection around row, moving outwards from reflection row
    for (a, b) in pattern[..(row + 1) * (row_length + PADDING)]
        .lines()
        .rev()
        .zip(pattern[(row + 1) * (row_length + PADDING)..].lines())
        .map(|(a, b)| (a.as_bytes(), b.as_bytes()))
    {
        for i in 0..row_length {
            if a[i] != b[i] {
                if one_diff {
                    return false;
                }
                one_diff = true;
            }
        }
    }
    one_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
#.##..##.\r
..#.##.#.\r
##......#\r
##......#\r
..#.##.#.\r
..##..##.\r
#.#.##.#.\r
\r
#...##..#\r
#....#..#\r
..##..###\r
#####.##.\r
#####.##.\r
..##..###\r
#....#..#";
        assert_eq!("400", process(input)?);
        Ok(())
    }
}
