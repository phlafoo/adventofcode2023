use crate::custom_error::AocError;
struct Mapping {
    source_range: std::ops::Range<i64>,
    offset: i64,
}

impl Mapping {
    fn map(&self, x: &mut i64) -> bool {
        if self.source_range.contains(x) {
            *x += self.offset;
            return true;
        }
        false
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // There are 7 mapping steps. For each step I will store the list of mappings that may apply.
    let mut mappings_collection: [Vec<Mapping>; 7] = std::array::from_fn(|_| vec![]);
    let mut mappings_index = 0;

    let mut lines = input.lines();

    // Get values that define seed ranges
    let mut seed_line_values = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .peekable();

    // Start at first mapping line
    lines.next();
    lines.next();

    // Collect all mappings
    while let Some(line) = lines.next() {
        // Skip to next mapping line if empty
        if line.is_empty() {
            lines.next();
            mappings_index += 1;
            continue;
        }
        // Grab values from mapping line
        let mut value_iter = line
            .split_ascii_whitespace()
            .map(|value| value.parse::<i64>().unwrap());

        let dest_start = value_iter.next().unwrap();
        let source_start = value_iter.next().unwrap();
        let range_length = value_iter.next().unwrap();

        // Save mapping for later use
        mappings_collection[mappings_index].push(Mapping {
            source_range: (source_start..source_start + range_length),
            offset: dest_start - source_start,
        });
    }

    let mut min = i64::MAX;

    // Iterate over seed ranges
    while seed_line_values.peek().is_some() {
        let range_start = seed_line_values.next().unwrap().parse::<i64>().unwrap();
        let range_length = seed_line_values.next().unwrap().parse::<i64>().unwrap();

        // Maps one seed at a time...
        for mut mapped_value in range_start..range_start + range_length {
            for mappings in mappings_collection.iter() {
                for mapping in mappings.iter() {
                    if mapping.map(&mut mapped_value) {
                        break;
                    }
                }
            }
            // Update min
            if mapped_value < min {
                min = mapped_value;
            }
        }
    }

    Ok(min.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
