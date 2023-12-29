use itertools::Itertools;

use crate::custom_error::AocError;

// brute force aka checking every possible combination
pub fn process_naive(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line_naive).sum::<i32>();
    Ok(result.to_string())
}

fn process_line_naive(line: &str) -> i32 {
    let (record, group_lengths) = line.split_ascii_whitespace().collect_tuple().unwrap();
    let group_lengths = group_lengths
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let mut current_total = 0;
    let mut q_indices = vec![];

    for (index, r) in record.chars().enumerate() {
        match r {
            '?' => q_indices.push(index),
            '#' => current_total += 1,
            _ => (),
        }
    }
    let target_total: i32 = group_lengths.iter().sum();
    let damaged_diff = target_total - current_total;
    let mut successes = 0;

    // iterate over all `damaged_diff`-length combinations of the '?' indices
    'comb: for comb in q_indices.iter().combinations(damaged_diff as usize) {
        // make a byte copy of record
        let mut record_bytes = record.as_bytes().to_vec();

        // set the '#' for this combination
        for &&i in &comb {
            record_bytes[i] = b'#'
        }
        let mut group_index = 0;
        let mut continguous = 0;

        // validate
        for r in &record_bytes {
            match r {
                b'#' => {
                    continguous += 1;
                }
                _ => {
                    if continguous > 0 {
                        if continguous != group_lengths[group_index] {
                            // invalid combination
                            continue 'comb;
                        }
                        // otherwise, go to next group
                        group_index += 1;
                        continguous = 0;
                    }
                }
            }
        }
        // another check here in case it ended on '#'
        if continguous > 0 && continguous != group_lengths[group_index] {
            continue 'comb;
        }
        successes += 1;
    }
    successes
}

// Dynamic programming approach. Mostly copied from https://github.com/mfornet/advent-of-code-2023/blob/main/src/bin/12.rs
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line).sum::<i32>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> i32 {
    let (record, group_lengths) = line.split_once(' ').unwrap();
    let group_lengths = group_lengths
        .split(',')
        .map(|n| n.parse::<usize>().unwrap());

    let record = record.as_bytes();

    let mut dp = vec![0; record.len() + 2];
    let mut dp_curr = vec![0; record.len() + 2];
    
    dp[0] = 1;
    for (i, _) in record.iter().take_while(|&&c| c != b'#').enumerate() {
        dp[i + 1] = 1;
    }

    for group_length in group_lengths {
        let mut contiguous = 0;
        dp_curr.fill(0);

        for (i, &r) in record.iter().enumerate() {
            contiguous = (r != b'.') as usize * (contiguous + 1);
            dp_curr[i + 2] += (r != b'#') as i32 * dp_curr[i + 1];

            if contiguous >= group_length
                && (i < group_length || record[i - group_length] != b'#')
            {
                dp_curr[i + 2] += dp[i + 1 - group_length]
            }
        }
        std::mem::swap(&mut dp, &mut dp_curr);
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process0() -> miette::Result<()> {
        let input = "???.### 1,1,3";
        assert_eq!("1", process(input)?);

        let input = "???..#.???.### 1,1,3";
        assert_eq!("6", process(input)?);

        let input = "???..#.???## 1,1,3";
        assert_eq!("4", process(input)?);

        let input = "???##????? 2,3";
        assert_eq!("3", process(input)?);

        let input = "???#??#?.?.???? 5,2";
        assert_eq!("7", process(input)?);
        /*
        1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0
        . ? ? ? # ? ? # ? . ? . ? ? ? ?
        0 0 0 0 0 0 1 2 1 2 2 2 2 2 2 2 2
        0 0 0 0 0 0 0 0 1 1 1 1 1 1 3 5 7
        */

        let input = "?###?????? 3,2,1";
        assert_eq!("3", process(input)?);

        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!("1", process(input)?);
        /*
        1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        . ? # ? # ? # ? # ? # ? # ? # ?
        0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1
        */
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
