use num::integer::lcm;
use std::collections::HashMap;

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    
    // Collect instructions
    let steps = lines.next().unwrap();

    lines.next();
    let mut start_nodes = vec![];

    // Collect nodes into hashmap for fast lookup
    let node_map = lines
        .map(|line| {
            let node = &line[0..3];
            let node_left = &line[7..10];
            let node_right = &line[12..15];
            // Find all starting nodes
            if node.ends_with('A') {
                start_nodes.push((node_left, node_right));
            }
            (node, (node_left, node_right))
        })
        .collect::<HashMap<_, _>>();

    // Count steps to find end node for each individual start node, then find LCM of those counts.
    // Iterating simultaneously would be too slow (20 trillion+ iterations).
    let lcm = start_nodes
        .iter()
        .map(|start_node| {
            let mut current_node = *start_node;
            let mut steps_count = 0;
            for step in steps.chars().cycle() {
                steps_count += 1;
                let next_key = match step {
                    'L' => current_node.0,
                    'R' => current_node.1,
                    _ => panic!("Invalid instruction"),
                };
                if next_key.ends_with('Z') {
                    break;
                }
                current_node = node_map[next_key];
            }
            steps_count
        })
        .reduce(|acc, count| lcm(acc, count) as u64)
        .unwrap();

    Ok(lcm.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
