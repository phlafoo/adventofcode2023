use crate::custom_error::AocError;

// 12 red cubes, 13 green cubes, and 14 blue cubes
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line).sum::<i32>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> i32 {
    let mut curr_count = 0;

    // limits defined in problem
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut token_iter = line.split_whitespace().skip(1);
    let game_num = token_iter.next().unwrap().split(':').next().unwrap();

    for token in token_iter {
        match token.parse::<i32>() {
            Ok(num) => curr_count = num,
            _ => {
                if match &token[..1] {
                    "r" => curr_count > red_limit,
                    "g" => curr_count > green_limit,
                    "b" => curr_count > blue_limit,
                    c => panic!("unexpected token {c}"),
                } {
                    return 0;
                }
            }
        }
    }
    game_num.parse::<i32>().unwrap()
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
