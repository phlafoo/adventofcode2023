use crate::custom_error::AocError;

// Uses shoelace algorithm and Pick's theorem
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut perimeter: i32 = 0;
    let mut det_sum = 0; // determinants sum
    let mut point = (0, 0);

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();

        let dir = match tokens.next().unwrap() {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("invalid direction"),
        };
        let dist = tokens.next().unwrap().parse::<i32>().unwrap();

        let next_point = (point.0 + (dist * dir.0), point.1 + (dist * dir.1));
        det_sum += (next_point.0 + point.0) * (next_point.1 - point.1); // see shoelace algorithm
        perimeter += dist;
        point = next_point;
    }
    let area = (det_sum / 2).abs();
    let interior_area = area - perimeter / 2 + 1; // see Pick's theorem

    let result = interior_area + perimeter;
    Ok(result.to_string())
}

// Using bytes is less ergonomic but much faster
pub fn process_bytes(input: &str) -> miette::Result<String, AocError> {
    let input = input.as_bytes();
    const LINE_LEN: usize = 14;

    let mut det_sum = 0; // determinants sum
    let mut perimeter = 0;
    let mut point = (0, 0);

    let mut index = 0;

    while index < input.len() {
        let dir = match input[index] {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => panic!("invalid direction"),
        };
        // check for 1 or 2 digit number
        let dist = if input[index + 4] == b' ' {
            index += 1;
            ((input[index + 1] - b'0') * 10 + input[index + 2] - b'0') as i32
        } else {
            (input[index + 2] - b'0') as i32
        };
        let next_point = (point.0 + (dist * dir.0), point.1 + (dist * dir.1));

        det_sum += (next_point.0 + point.0) * (next_point.1 - point.1); // see shoelace algorithm
        perimeter += dist;

        point = next_point;
        index += LINE_LEN;
    }
    let area = (det_sum / 2).abs();
    let interior_area = area - perimeter / 2 + 1; // see Pick's theorem

    let result = interior_area + perimeter;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        assert_eq!("62", process_bytes(input)?);
        Ok(())
    }
}
