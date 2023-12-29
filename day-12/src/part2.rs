use itertools::Itertools;

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line).sum::<i128>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> i128 {
    let (record, group_lengths) = line.split_once(' ').unwrap();
    let group_lengths = group_lengths
        .split(',')
        .map(|n| n.parse::<usize>().unwrap());

    let group_lengths = std::iter::once(group_lengths).cycle().take(5).flatten();

    let record = std::iter::once(record).cycle().take(5).join("?");
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
            dp_curr[i + 2] += (r != b'#') as i128 * dp_curr[i + 1];

            if contiguous >= group_length && (i < group_length || record[i - group_length] != b'#')
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

        let input = ".??..??...?##. 1,1,3";
        assert_eq!("16384", process(input)?);

        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!("1", process(input)?);

        let input = "????.#...#... 4,1,1";
        assert_eq!("16", process(input)?);

        let input = "????.######..#####. 1,6,5";
        assert_eq!("2500", process(input)?);

        let input = "?###???????? 3,2,1";
        assert_eq!("506250", process(input)?);

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
        assert_eq!("525152", process(input)?);
        Ok(())
    }
}