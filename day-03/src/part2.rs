use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // row length is 10 for test case
    let row_length = input.find('\n').unwrap() + 1;
    let mut total = 0;
    let mut part_numbers = [0, 0];

    'symbols: for (symbol_index, _) in input.match_indices('*') {
        let mut part_number_index = 0;

        // check left
        for (index, c) in input[..symbol_index].chars().rev().chain(".".chars()).enumerate() {
            match c {
                n if n.is_ascii_digit() => continue,
                _ => {
                    if let Ok(num) = input[symbol_index - index..symbol_index].parse::<i32>() {
                        part_numbers[part_number_index] = num;
                        part_number_index += 1;
                    }
                    break;
                }
            }
        }

        // check right
        for (index, c) in input[symbol_index + 1..].chars().chain(".".chars()).enumerate() {
            match c {
                n if n.is_ascii_digit() => continue,
                _ => {
                    if let Ok(num) = input[symbol_index + 1..index + symbol_index + 1].parse::<i32>() {
                        part_numbers[part_number_index] = num;
                        part_number_index += 1;
                        // at this point we may have 2 part numbers already
                        if update_total(&mut total, &part_numbers, &part_number_index) {
                            continue 'symbols;
                        }
                    }
                    break;
                }
            }
        }

        

        // check top starting with top left and moving left
        if symbol_index < row_length {
            continue;
        }
        'outer: for (index_left, c) in input[..symbol_index - row_length].chars().rev().chain(".".chars()).enumerate() {
            match c {
                n if n.is_ascii_digit() => continue,
                _ => {
                    // Once a non-number is found, start moving right starting at above the symbol (middle)
                    for (index_right, c) in input[symbol_index - row_length..].chars().enumerate() {
                        match c {
                            n if n.is_ascii_digit() => continue,
                            _ => {
                                // after checking top left and top middle, we have (at most) 1 number to add
                                if let Ok(num) = input[symbol_index - row_length - index_left..symbol_index - row_length + index_right].parse::<i32>() {
                                    part_numbers[part_number_index] = num;
                                    part_number_index += 1;
                                    if update_total(&mut total, &part_numbers, &part_number_index) {
                                        continue 'symbols;
                                    }
                                }
                                // now check top right if we haven't already
                                if index_right < 2 {
                                    for (index_right, c) in input[symbol_index - row_length + 1..].chars().enumerate() {
                                        match c {
                                            n if n.is_ascii_digit() => continue,
                                            _ => {
                                                if let Ok(num) = input[symbol_index - row_length + 1..index_right + symbol_index - row_length + 1].parse::<i32>() {
                                                    part_numbers[part_number_index] = num;
                                                    part_number_index += 1;
                                                    if update_total(&mut total, &part_numbers, &part_number_index) {
                                                        continue 'symbols;
                                                    }
                                                }
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        // check bottom starting with bottom left
        if symbol_index + row_length > input.len(){
            continue;
        }
        'outer: for (index_left, c) in input[..symbol_index + row_length].chars().rev().enumerate() {
            match c {
                n if n.is_ascii_digit() => continue,
                _ => {
                    // Once a non-number is found, start moving right starting at below the symbol (middle)
                    for (index_right, c) in input[symbol_index + row_length..].chars().chain(".".chars()).enumerate() {
                        match c {
                            n if n.is_ascii_digit() => continue,
                            _ => {
                                // after checking bottom left and bottom middle, we have (at most) 1 number to add
                                if let Ok(num) = input[symbol_index + row_length - index_left..symbol_index + row_length + index_right].parse::<i32>() {
                                    part_numbers[part_number_index] = num;
                                    part_number_index += 1;
                                    if update_total(&mut total, &part_numbers, &part_number_index) {
                                        continue 'symbols;
                                    }
                                }
                                // now check bottom right if we haven't already
                                if index_right < 2 {
                                    for (index_right, c) in input[symbol_index + row_length + 1..].chars().chain(".".chars()).enumerate() {
                                        match c {
                                            n if n.is_ascii_digit() => continue,
                                            _ => {
                                                if let Ok(num) = input[symbol_index + row_length + 1..index_right + symbol_index + row_length + 1].parse::<i32>() {
                                                    part_numbers[part_number_index] = num;
                                                    part_number_index += 1;
                                                    if update_total(&mut total, &part_numbers, &part_number_index) {
                                                        continue 'symbols;
                                                    }
                                                }
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(total.to_string())
}

#[inline]
fn update_total(total: &mut i32, part_numbers: &[i32; 2], index: &usize) -> bool {
    if *index == 2 {
        *total += part_numbers[0] * part_numbers[1];
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
