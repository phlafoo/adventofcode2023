use std::fs;

fn main() {
    let input: String = fs::read_to_string("data/day1_input.txt").unwrap();

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    // Iterate line by line and accumulate result
    input.split('\n').fold(0, |acc, word| {
        let digits = word.matches(char::is_numeric).collect::<Vec<_>>();
        acc + digits.first().unwrap().parse::<i32>().unwrap() * 10
            + digits.last().unwrap().parse::<i32>().unwrap()
    })
}

// For part two
#[derive(Debug, Default, Clone, Copy)]
struct Match {
    index: usize,
    digit: usize,
}

fn part_two(input: &str) -> i32 {
    // Index represents numerical value
    let spelled_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Iterate line by line and accumulate result
    input.split('\n').fold(0, |acc, word| {
        // First we find numerical chars only, keeping track of their index in the line
        let mut matches = word
            .match_indices(char::is_numeric)
            .map(|(index, char)| Match {
                index,
                digit: char.parse::<usize>().unwrap(),
            })
            .collect::<Vec<_>>();

        // Then we find the spelled versions
        for (num, num_string) in spelled_digits.iter().enumerate() {
            matches.append(
                word.match_indices(num_string)
                    .map(|(index, _)| Match { index, digit: num })
                    .collect::<Vec<_>>()
                    .as_mut(),
            );
        }

        // Since we tracked the index, we just need to find the values with the smallest and largest
        // indices as our first and second digits respectively
        let first_digit = matches
            .iter()
            .min_by(|a, b| a.index.cmp(&b.index))
            .copied()
            .unwrap_or_default()
            .digit;

        let second_digit = matches
            .iter()
            .max_by(|a, b| a.index.cmp(&b.index))
            .copied()
            .unwrap_or_default()
            .digit;

        acc + first_digit * 10 + second_digit
    }) as i32
}
