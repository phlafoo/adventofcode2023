use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut input_split = input.split("\n\n");

    let workflows_list = input_split.next().unwrap().as_bytes();
    let parts_list = input_split.next().unwrap().as_bytes();

    let largest_index = hash("zzz".as_bytes());
    let mut workflows = vec![vec![]; largest_index + 1];

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
                _ => Target::Workflow(hash(&rule[target_start..])),
            };
            workflows[index].push(Rule::new(condition, target));
        }
    }

    let start_workflow = hash("in".as_bytes());
    let mut result = 0;

    'parts: for line in parts_list.split(|&c| c == b'\n') {
        let mut workflow_index = start_workflow;
        let line = &line[1..line.len() - 1];
        let mut rankings = [0; 4];

        for (index, rating) in line.split(|&c| c == b',').enumerate() {
            rankings[index] = rating[2..]
                .iter()
                .fold(0, |acc, d| (acc * 10) + (d - b'0') as u32);
        }
        'rules: loop {
            for rule in &workflows[workflow_index] {
                if rule.condition.is_satisfied_for(rankings) {
                    match rule.target {
                        Target::Workflow(i) => {
                            workflow_index = i;
                            continue 'rules;
                        }
                        Target::Accept => {
                            result += rankings.iter().sum::<u32>();
                            continue 'parts;
                        }
                        Target::Reject => continue 'parts,
                    }
                }
            }
        }
    }
    Ok(result.to_string())
}

/// Converts workflow name to index
fn hash(workflow_name: &[u8]) -> usize {
    workflow_name
        .iter()
        .fold(0, |acc, d| (acc * 26) + (d - b'a') as usize)
}

fn get_condition(condition: &[u8]) -> Condition<u32> {
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
struct Rule<T: Ord> {
    condition: Condition<T>,
    target: Target,
}

impl<T: Ord> Rule<T> {
    pub fn new(condition: Condition<T>, target: Target) -> Self {
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

#[derive(Clone, Debug)]
enum Condition<T: Ord> {
    // stores index and value to compare against
    GreaterThan(usize, T),
    LessThan(usize, T),
    None,
}

impl<T: Ord> Condition<T> {
    pub fn is_satisfied_for(&self, rankings: [T; 4]) -> bool {
        match self {
            Self::GreaterThan(i, c) => rankings[*i] > *c,
            Self::LessThan(i, c) => rankings[*i] < *c,
            Self::None => true,
        }
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
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}
