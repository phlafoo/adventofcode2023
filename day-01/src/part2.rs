use crate::custom_error::AocError;

#[derive(Debug, Default, Clone, Copy)]
struct Match {
    index: usize,
    digit: usize,
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Index represents numerical value
    let spelled_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut matches = [Match::default(); 10];

    // Iterate line by line and accumulate result
    let result = input.lines().fold(0, |acc, word| {
        // First we find numerical chars only, keeping track of their index in the line
        let mut matches_index = 0;
        word.match_indices(char::is_numeric)
            .map(|(index, char)| Match {
                index,
                digit: char.parse::<usize>().unwrap(),
            })
            .zip(matches.iter_mut())
            .for_each(|(w, m)| {
                matches_index += 1;
                *m = w;
            });

        // Then we find the spelled versions
        for (num, num_string) in spelled_digits.iter().enumerate() {
            word.match_indices(num_string)
                .map(|(index, _)| Match { index, digit: num })
                .zip(matches[matches_index..].iter_mut())
                .for_each(|(w, m)| {
                    matches_index += 1;
                    *m = w;
                });
        }

        // Since we tracked the index, we just need to find the values with the smallest and largest
        // indices as our first and second digits respectively
        let first_digit = matches[..matches_index]
            .iter()
            .min_by(|a, b| a.index.cmp(&b.index))
            .copied()
            .unwrap_or_default()
            .digit;

        let second_digit = matches[..matches_index]
            .iter()
            .max_by(|a, b| a.index.cmp(&b.index))
            .copied()
            .unwrap_or_default()
            .digit;
        // dbg!(matches_index);
        // dbg!(first_digit);
        // dbg!(second_digit);

        acc + first_digit * 10 + second_digit
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
