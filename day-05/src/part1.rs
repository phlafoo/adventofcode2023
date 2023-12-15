use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Map each seed to its final "location" number then find the minimum
    let result = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|seed| {
            // `mapped_value` is reused for each mapping step
            let mut mapped_value = seed.parse::<i64>().unwrap();

            // Start at first mapping line
            let mut lines = input.lines().skip(3).peekable();

            while let Some(line) = lines.next() {
                // Skip to next mapping line if empty
                if line.is_empty() {
                    lines.next();
                    continue;
                }
                // Grab values from mapping line
                let mut value_iter = line
                    .split_ascii_whitespace()
                    .map(|value| value.parse::<i64>().unwrap());

                let dest_start = value_iter.next().unwrap();
                let source_start = value_iter.next().unwrap();
                let range_length = value_iter.next().unwrap();

                // Determine if input value is found in mapping range of current line
                if (source_start..source_start + range_length).contains(&mapped_value) {
                    mapped_value = dest_start + mapped_value - source_start;

                    // Value has been updated, skip to the next map
                    while let Some(useless_line) = lines.peek() {
                        if useless_line.is_empty() {
                            break;
                        }
                        lines.next();
                    }
                }
            }
            mapped_value
        })
        .min()
        .unwrap();

    Ok(result.to_string())
}

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

pub fn process_optimized(input: &str) -> miette::Result<String, AocError> {
    // There are 7 mapping steps. For each step I will store the list of mappings that may apply.
    let mut mappings_collection: [Vec<Mapping>; 7] =
        std::array::from_fn(|_| vec![]);
    let mut mappings_index = 0;

    let mut lines = input.lines();

    // Get seeds
    let seeds = lines.next().unwrap().split_ascii_whitespace().skip(1);

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

    // Map each seed to its final "location" number then find the minimum
    let result = seeds
        .map(|seed| {
            let mut mapped_value = seed.parse::<i64>().unwrap();
            for mappings in mappings_collection.iter() {
                for mapping in mappings.iter() {
                    if mapping.map(&mut mapped_value) {
                        break;
                    }
                }
            }
            mapped_value
        })
        .min()
        .unwrap();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        //79 14 55 13
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
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
