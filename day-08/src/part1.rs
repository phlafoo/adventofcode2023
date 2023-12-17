use std::collections::HashMap;

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Collect instructions
    let mut lines = input.lines();
    let steps = lines.next().unwrap();

    lines.next();

    // Collect nodes into hashmap for fast lookup
    let node_map = lines
        .map(|line| {
            let node = &line[0..3];
            let node_left = &line[7..10];
            let node_right = &line[12..15];
            (node, (node_left, node_right))
        })
        .collect::<HashMap<_, _>>();

    // Start at "AAA"
    let mut current_node = node_map["AAA"];
    let mut steps_count = 0;

    for step in steps.chars().cycle() {
        steps_count += 1;
        let next_key = match step {
            'L' => current_node.0,
            'R' => current_node.1,
            _ => panic!("Invalid instruction"),
        };
        // Stop at "ZZZ"
        if next_key == "ZZZ" {
            break;
        }
        current_node = node_map[next_key];
    }

    Ok(steps_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input1 = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let input2 = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input1)?);
        assert_eq!("6", process(input2)?);
        Ok(())
    }
}
