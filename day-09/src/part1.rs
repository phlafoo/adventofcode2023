use crate::custom_error::AocError;

/// Naive approach
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|line| {
            // Parse line
            let mut values = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("valid integer string"))
                .collect::<Vec<_>>();
            let mut prediction = *values.last().unwrap();
            loop {
                // println!("{:?}", values);
                // Iterate over neighbors
                values = values
                    .iter()
                    .zip(values.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>();
                // If all zeros we are done
                if values.iter().all(|v| *v == 0) {
                    return prediction;
                }
                prediction += values.last().unwrap();
            }
        })
        .sum::<i32>();

    Ok(result.to_string())
}

/// Reuse a single array instead of allocating vectors
pub fn process_optimized(input: &str) -> miette::Result<String, AocError> {
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

            let mut prediction = values[ROW_LEN - 1];
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
                prediction += values[values_len - 1];
            }
        })
        .sum::<i32>();

    Ok(result.to_string())
}

/// Depth first traversal
pub fn process_optimized_dft(input: &str) -> miette::Result<String, AocError> {
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

            let mut prediction = values[ROW_LEN - 1];

            for i in (0..ROW_LEN).rev() {
                for j in i..ROW_LEN {
                    values[j] -= values[j - 1];
                }
                match values[ROW_LEN - 1] {
                    0 => break,
                    n => prediction += n,
                }
            }
            prediction
        })
        .sum::<i32>();

    Ok(result.to_string())
}

/// Using binomial coefficients.
/// For example for a row length of 5:
///     prediction = a - 5b + 10c - 10d + 5e
pub fn process_bc(input: &str) -> miette::Result<String, AocError> {
    // Pre-computed binomial coefficients
    let bin_coeffs = [
        1, -21, 210, -1330, 5985, -20349, 54264, -116280, 203490, -293930, 352716, -352716, 293930,
        -203490, 116280, -54264, 20349, -5985, 1330, -210, 21,
    ];
    // for test input:
    // let bin_coeffs = [-1, 6, -15, 20, -15, 6];

    // Or compute at runtime
    // // const ROW_LEN: usize = 6; // for test input
    // const ROW_LEN: usize = 21;
    // let n = ROW_LEN + 1;

    // let mut bin_coeffs = [0_i32; ROW_LEN];
    // bin_coeffs[0] = 1;
    // for k in 1..=(n / 2) {
    //     let bn = num::integer::binomial(ROW_LEN, k) as i32;
    //     bin_coeffs[ROW_LEN - k] = bn;
    //     bin_coeffs[k] = bn;
    // }

    // let mut sign = if ROW_LEN % 2 == 0 { -1 } else { 1 };
    // for bn in &mut bin_coeffs {
    //     *bn *= sign;
    //     sign *= -1;
    // }
    // println!("{:?}", bin_coeffs);

    let result = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("valid integer string"))
                .enumerate()
                .fold(0, |acc, (i, v)| acc + bin_coeffs[i] * v)
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
        assert_eq!("114", process(input)?);
        Ok(())
    }
}

/*
process_optimized
[day-09\src\part1.rs:64] total_steps = 30747

process_optimized_bft
[day-09\src\part1.rs:98] total_steps = 16440
*/
