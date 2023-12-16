use crate::custom_error::AocError;
use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Get time and distance
    let (time, distance) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    // Find range
    let discriminant = f64::sqrt((time * time - 4 * distance) as f64);
    let range_start = (0.5 * (time as f64 - discriminant)).floor() as u32 + 1;
    let range_end = (0.5 * (time as f64 + discriminant)).ceil() as u32;
    let result = range_end - range_start;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
