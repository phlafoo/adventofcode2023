use crate::custom_error::AocError;

fn get_direction(dir: &str) -> (i128, i128) {
    match dir {
        "0" => (1, 0),  // right
        "1" => (0, 1),  // down
        "2" => (-1, 0), // left
        "3" => (0, -1), // up
        _ => panic!("invalid direction"),
    }
}

// Uses shoelace algorithm and Pick's theorem
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut perimeter: i128 = 0;
    let mut det_sum = 0; // determinants sum
    let mut vert = (0, 0);

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace().skip(2);
        let hex_code = &tokens.next().unwrap()[2..];
        let dist = i128::from_str_radix(&hex_code[..hex_code.len() - 2], 16).unwrap();
        let dir = get_direction(&hex_code[hex_code.len() - 2..hex_code.len() - 1]);

        let next_vert = (vert.0 + (dist * dir.0), vert.1 + (dist * dir.1));
        det_sum += (next_vert.0 + vert.0) * (next_vert.1 - vert.1); // see shoelace algorithm
        perimeter += dist;
        vert = next_vert;
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

    let mut index = 6;

    while index < input.len() {
        if input[index - 1] != b'#' {
            index += 1;
        }
        let dir = match input[index + 5] {
            b'0' => (1, 0),  // right
            b'1' => (0, 1),  // down
            b'2' => (-1, 0), // left
            b'3' => (0, -1), // up
            c => panic!("invalid direction: {}", c as char),
        };
        // manually parse hex to decimal
        let dist = input[index..index + 5].iter().fold(0, |acc, &h| {
            (acc * 16) + if h < b'a' { h - b'0' } else { h - b'a' + 10 } as i64
        });
        let next_point = (point.0 + (dist * dir.0), point.1 + (dist * dir.1));

        det_sum += (next_point.0 + point.0) * (next_point.1 - point.1); // see shoelace algorithm
        perimeter += dist;

        point = next_point;
        index += LINE_LEN
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
        assert_eq!("952408144115", process(input)?);
        assert_eq!("952408144115", process_bytes(input)?);
        Ok(())
    }
}
