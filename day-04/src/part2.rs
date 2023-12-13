use crate::custom_error::AocError;
use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Using a fixed-size array instead of a vec somehow makes the performance worse.
    let num_games = input.lines().count();
    let mut num_copies = vec![1_u32; num_games];

    let mut result = 0;

    for (game_index, line) in input.lines().enumerate() {
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
        let matches_count = winners
            .filter(|winner| my_numbers.clone().contains(winner))
            .count();

        // Get the final count of game instances and add to result
        let instances = num_copies[game_index];
        result += instances;

        // Update instance counts for subsequent games
        for i in 1..=matches_count {
            num_copies[game_index + i] += instances;
        }
    };
  
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
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
