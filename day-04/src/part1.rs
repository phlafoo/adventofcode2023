use crate::custom_error::AocError;
use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Accumulate the result of each line
    let result = input.lines().fold(0, |total, line| {
        // Parse and collect each line into two iterators. One for each number list.
        let (winners, my_numbers) = line
            .split_once(':')
            .unwrap()
            .1
            .split('|')
            .map(|list| list.split_whitespace())
            .collect_tuple()
            .unwrap();

        // Count how many winning numbers appear in "my numbers"
        match winners
            .filter(|winner| my_numbers.clone().contains(winner))
            .count()
        {
            0 => total,
            c => total + 2_u32.pow(c as u32 - 1),
        }
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
