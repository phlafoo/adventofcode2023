use crate::custom_error::AocError;

#[derive(Debug)]
struct Rule {
    start: i64,
    end: i64,
    offset: i64,
}

impl Rule {
    fn map(&self, x: &mut i64) -> bool {
        if (self.start..self.end).contains(x) {
            *x += self.offset;
            return true;
        }
        false
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // There are 7 mapping steps. For each step I will store the list of mappings that may apply.
    let mut mappings: [Vec<Rule>; 7] = std::array::from_fn(|_| vec![]);
    let mut mapping_index = 0;

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
            mapping_index += 1;
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
        mappings[mapping_index].push(Rule {
            start: source_start,
            end: source_start + range_length,
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
            for mapping in mappings.iter() {
                for rule in mapping.iter() {
                    if rule.map(&mut mapped_value) {
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

/// The idea here is to process per seed range instead of per seed
pub fn process_optimized(input: &str) -> miette::Result<String, AocError> {
    // There are 7 mapping steps. For each step I will store the list of mappings that may apply.
    let mut mappings: [Vec<Rule>; 7] = std::array::from_fn(|_| vec![]);
    let mut mappings_index = 0;

    let mut lines = input.lines();

    // Get values that define seed ranges
    let mut seed_line_values = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
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
        mappings[mappings_index].push(Rule {
            start: source_start,
            end: source_start + range_length,
            offset: dest_start - source_start,
        });
    }

    let mut ranges = vec![];

    // Iterate over seed ranges and save in `ranges`
    while seed_line_values.peek().is_some() {
        let range_start = seed_line_values.next().unwrap();
        let range_length = seed_line_values.next().unwrap();
        ranges.push(range_start..range_start + range_length);
    }

    let mut range_index = 0;
    let mut mapped_ranges = vec![];

    for mapping in mappings.iter() {
        for rule in mapping.iter() {
            // Getting ranges.len() here and doing `while range_index < ranges_len` instead of `loop`
            // does slightly reduce number of iterations but the performance is slightly worse...
            loop {
                let Some(input_range) = ranges.get(range_index) else {
                    range_index = 0;
                    break;
                };
                
                // Grabbing the variables in this way gives ~5-10% performance improvement
                let input_start = input_range.start;
                let input_end = input_range.end;
                let rule_start = &rule.start;
                let rule_end = &rule.end;
                let offset = &rule.offset;
                
                // ## Cases Legend
                // input  {}
                // rule   []
                // case 1: {}[] => no changes
                // case 2: []{} => no changes
                // case 3: [{}] => m{}          (remove input)
                // case 4: [{]} => m{], ]}      (replace input)
                // case 5: {[}] => {[ , m[}     (replace input)
                // case 6: {[]} => {[ , m[], ]} (replace input and push)

                // case 1 and 2
                if input_start >= *rule_end || input_end <= *rule_start {
                    range_index += 1;
                    continue;
                }

                match (input_start >= *rule_start, input_end <= *rule_end) {
                    (true, true) => {
                        // case 3
                        ranges.swap_remove(range_index);
                        mapped_ranges
                            .push(input_start + offset..input_end + offset);
                    }
                    (true, false) => {
                        // case 4
                        mapped_ranges.push(input_start + offset..rule_end + offset);
                        ranges[range_index] = *rule_end..input_end;
                        // Incrementing range_index for case 4 and 5 reduces redundant range checks
                        // but for some reason the performance is slightly worse
                        // range_index += 1;
                    }
                    (false, true) => {
                        // case 5
                        ranges[range_index] = input_start..*rule_start;
                        mapped_ranges.push(rule_start + offset..input_end + offset);
                        // range_index += 1;
                    }
                    (false, false) => {
                        // case 6
                        ranges[range_index] = input_start..*rule_start;
                        mapped_ranges.push(rule_start + offset..rule_end + offset);
                        ranges.push(*rule_end..input_end);
                        range_index += 1;
                    }
                }
            }
        }
        // `ranges` now contains only inputs that did not match any rule, so their values don't
        // change for the next step.
        ranges.append(&mut mapped_ranges);
    }
    let min = ranges.iter().map(|r| r.start).min().unwrap();

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
