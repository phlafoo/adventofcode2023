use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let width = input.find('\n').unwrap();
    let height = width + 1;
    let input = input.as_bytes();

    let mut column_loads = vec![height - 1; width];
    let mut result = 0;
    
    for (row_index, row) in input.chunks(width + 1).enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            match tile {
                b'#' => column_loads[col_index] = height - row_index - 2,
                b'O' => {
                    result += column_loads[col_index];
                    column_loads[col_index] -= 1;
                }
                _ => (),
            }
        }
    }
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
