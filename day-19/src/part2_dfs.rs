use std::ops::Range;

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut input_split = input.split("\n\n");

    let workflows_list = input_split.next().unwrap().as_bytes();

    let largest_index = hash("zzz".as_bytes());
    let mut workflows = vec![Workflow::default(); largest_index + 1];

    // parse workflows
    for line in workflows_list.split(|&c| c == b'\n') {
        let divider = line.iter().position(|&c| c == b'{').unwrap();
        let index = hash(&line[..divider]);

        for rule in line[divider + 1..line.len() - 1].split(|&c| c == b',') {
            let (condition, target_start) = match rule.iter().position(|&c| c == b':') {
                Some(colon_index) => (get_condition(&rule[..colon_index]), colon_index + 1),
                None => (Condition::None, 0),
            };
            let target = match rule[target_start] {
                b'A' => Target::Accept,
                b'R' => Target::Reject,
                _ => {
                    let target_index = hash(&rule[target_start..]);
                    Target::Workflow(target_index)
                }
            };
            workflows[index].add_rule(condition, target);
        }
    }
    let start_workflow = hash("in".as_bytes());
    let mut result = 0;

    const DEFAULT_RANGE: Range<u32> = 1..4000 + 1;
    let mut stack = vec![(start_workflow, [DEFAULT_RANGE; 4])];

    // dfs
    while let Some((workflow_index, mut ranges)) = stack.pop() {
        let workflow = &workflows[workflow_index];
        for rule in &workflow.rules {
            let mut condition = rule.condition;
            let mut ranges_clone = ranges.clone();

            if !condition.apply(&mut ranges_clone) {
                break;
            }
            match rule.target {
                Target::Accept => {
                    result += ranges_clone
                        .iter()
                        .fold(1, |acc, r| acc * (r.end - r.start) as u128);
                }
                Target::Workflow(i) => {
                    stack.push((i, ranges_clone))
                }
                Target::Reject => (),
            }
            condition.negate();
            if !condition.apply(&mut ranges) {
                break;
            }
        }
    }

    Ok(result.to_string())
}

#[derive(Default, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn add_rule(&mut self, condition: Condition, target: Target) {
        self.rules.push(Rule::new(condition, target));
    }
}

/// Converts workflow name to index
fn hash(workflow_name: &[u8]) -> usize {
    workflow_name
        .iter()
        .fold(0, |acc, d| (acc * 26) + (d - b'a') as usize)
}

/// for debugging
#[allow(unused)]
fn unhash(mut workflow_index: usize) -> String {
    let mut bytes = vec![];
    while workflow_index != 0 {
        let q = workflow_index / 26;
        bytes.push((workflow_index % 26) as u8 + b'a');
        workflow_index = q;
    }
    bytes.reverse();
    std::str::from_utf8(&bytes).unwrap().to_string()
}

fn get_condition(condition: &[u8]) -> Condition {
    let value = condition[2..]
        .iter()
        .fold(0, |acc, d| (acc * 10) + (d - b'0') as u32);
    let index = match condition[0] {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!("Must be in \"xmas\""),
    };
    match condition[1] {
        b'<' => Condition::LessThan(index, value),
        b'>' => Condition::GreaterThan(index, value),
        _ => unreachable!("Must be either '>' or '<'"),
    }
}

#[derive(Clone, Debug)]
struct Rule {
    condition: Condition,
    target: Target,
}

impl Rule {
    pub fn new(condition: Condition, target: Target) -> Self {
        Rule { condition, target }
    }
}

#[derive(Clone, Debug)]
enum Target {
    // stores index
    Workflow(usize),
    Accept,
    Reject,
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    // stores index and value to compare against
    GreaterThan(usize, u32),
    LessThan(usize, u32),
    None,
}

impl Condition {
    pub fn negate(&mut self) {
        *self = match self {
            Self::GreaterThan(i, c) => Self::LessThan(*i, *c + 1),
            Self::LessThan(i, c) => Self::GreaterThan(*i, *c - 1),
            Self::None => Self::None,
        }
    }

    pub fn apply(&self, ranges: &mut [Range<u32>; 4]) -> bool {
        let range = match self {
            Self::GreaterThan(i, c) => {
                ranges[*i].start = ranges[*i].start.max(*c + 1);
                &ranges[*i]
            }
            Self::LessThan(i, c) => {
                ranges[*i].end = ranges[*i].end.min(*c);
                &ranges[*i]
            }
            Self::None => return true,
        };
        if range.start > range.end {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
