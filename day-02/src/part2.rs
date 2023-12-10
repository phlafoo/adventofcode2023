use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line).sum::<i32>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> i32 {
    let mut curr_count = 0;

    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;

    for token in line.split_whitespace().skip(2) {
        match token.parse::<i32>() {
            Ok(num) => curr_count = num,
            _ => {
                match &token[..1] {
                    "r" => red_max = red_max.max(curr_count),
                    "g" => green_max = green_max.max(curr_count),
                    "b" => blue_max = blue_max.max(curr_count),
                    c => panic!("unexpected token {c}"),
                }
            }
        }
    }
    red_max * green_max * blue_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
