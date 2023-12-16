use crate::custom_error::AocError;
use itertools::Itertools;
/*
Distance can be calculated as
    d = (t - b) * b
where t = time and b = how long button is held

Replacing distance (d) with distance record (r) we have the following inequality
    r < (t - b) * b
where we want to find the range of (b) where this holds.

Rearrange into a quadratic equation
    0 > b^2 - tb + r
Then applying quadratic formula we get
    b < 0.5 * (t + sqrt(t^2 - 4r))
    b > 0.5 * (t - sqrt(t^2 - 4r))

So each race becomes a simple constant time calculation
*/
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Get iterators over times and distances
    let (times, distances) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
        })
        .collect_tuple()
        .unwrap();

    let result = times.zip(distances).fold(1, |acc, (time, distance)| {
        // Find range and collect result in `acc`
        let discriminant = f32::sqrt((time * time - 4 * distance) as f32);
        let range_start = (0.5 * (time as f32 - discriminant)).floor() as u32 + 1;
        let range_end = (0.5 * (time as f32 + discriminant)).ceil() as u32;
        acc * (range_end - range_start)
    });

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
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
