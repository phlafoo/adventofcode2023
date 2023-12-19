use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // const ROW_LEN: usize = 6; // for test input
    const ROW_LEN: usize = 21;

    let result = input
        .lines()
        .map(|line| {
            let mut values = [0; ROW_LEN];

            // Parse line
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("valid integer string"))
                .enumerate()
                .for_each(|(i, v)| {
                    values[i] = v;
                });

            let mut prediction = values[0];
            let mut sign = -1;
            let mut values_len = ROW_LEN;

            loop {
                for i in 1..values_len {
                    values[i - 1] = values[i] - values[i - 1];
                }
                // The last value gets ignored after each step
                values_len -= 1;
                if values.iter().take(values_len).all(|v| *v == 0) {
                    return prediction;
                }
                // Just need to swap signs on each step. This is because:
                // (x_1 - (x_2 - (x_3 - (...(x_n-1 - x_n)...))))
                // is equivalent to
                // x_1 - x_2 + x_3 - ... + x_n-1 - x_n
                prediction += values[0] * sign;
                sign *= -1;
            }
        })
        .sum::<i32>();

    Ok(result.to_string())
}

/// This doesn't generalize to all inputs but it does work for the given input
pub fn process_dft(input: &str) -> miette::Result<String, AocError> {
    // const ROW_LEN: usize = 6; // for test input
    const ROW_LEN: usize = 21;

    let result = input
        .lines()
        .map(|line| {
            let mut values = [0; ROW_LEN];

            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("valid integer string"))
                .enumerate()
                .for_each(|(i, v)| {
                    values[i] = v;
                });

            let mut prediction = values[0];
            let mut sign = -1;

            for i in 0..(ROW_LEN - 1) {
                for j in (0..=i).rev() {
                    values[j] = values[j + 1] - values[j];
                }
                // Checking the first 3 values is sufficient for my input.
                match values[0..=2] {
                    [0, 0, 0] => break,
                    _ => {
                        prediction += values[0] * sign;
                    },
                }
                sign *= -1;
            }
            prediction
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
