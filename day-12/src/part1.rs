use std::mem::swap;

use itertools::Itertools;

use crate::custom_error::AocError;
/*
6 choose 4
111100
011110
001111
100111
110011
111001

111010
011101
101110
010111
101011
110101

110110
011011
101101

6 choose 3
111000



111000
011100
001110
000111
100011
110001

110100
011010
001101
100110
010011
101001

011001
101100
010110
001011
100101
110010

101010
010101


5 choose 3
11100
01110
00111
10011
11001

10101
11010
01101
10110
01011

5 choose 2
11000
01100
00110
00011
10001

01001
10100
01010
00101
10010

4 choose 3
1110
0111
1011
1101

4 choose 2
1100
0110
0011
1001

0101
1010

*/
pub fn process(input: &str) -> miette::Result<String, AocError> {

    // const SIZE: usize = 5;
    // let d = 3;
    // for c in (0..SIZE).combinations(d) {
    //     let mut arr = [0; SIZE];
    //     for i in c {
    //         arr[i] = 1;
    //     }
    //     println!("{:?}", arr);
    // }

    let result = input.lines().map(process_line).sum::<i32>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> i32 {
    let (record, group_lengths) = line.split_ascii_whitespace().collect_tuple().unwrap();
    let group_lengths = group_lengths
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // let mut record_bytes = record.as_bytes().to_vec();
    // println!("{:?}", record_bytes.iter().map(|r| *r as char).collect::<Vec<char>>());

    let target_total: i32 = group_lengths.iter().sum();
    let mut current_total = 0;
    let mut q_indices = vec![];
    // let mut q_count = 0;
    
    let mut successes = 0;
    
    for (index, r) in record.chars().enumerate() {
        match r {
            '?' => q_indices.push(index),
            '#' => current_total += 1,
            _ => ()
        }
    }
    let damaged_diff = target_total - current_total;
    // println!("{:?}", q_indices.iter().combinations(damaged_diff as usize).collect::<Vec<_>>());

    'comb: for comb in q_indices.iter().combinations(damaged_diff as usize) {
        let mut record_bytes = record.as_bytes().to_vec();
        for &&i in &comb {
            record_bytes[i] = b'#'
        }
        // println!("\ncomb: {:?}", comb);
        // println!("reco: {:?}", record_bytes.iter().map(|r| *r as char).collect::<Vec<char>>());
        let mut group_index = 0;
        let mut continguous = 0;
        for r in &record_bytes {
            // dbg!(continguous, group_index);
            // println!();
            match r {
                b'#' => {
                    continguous += 1;
                    
                },
                _ => {
                    if continguous > 0 {
                        if continguous != group_lengths[group_index] {
                            continue 'comb;
                        }
                        group_index += 1;
                        continguous = 0;
                    }
                }
            }
        }
        if continguous > 0 && continguous != group_lengths[group_index] {
            continue 'comb;
        }
        // println!("SUCCESS: {:?}", record_bytes.iter().map(|r| *r as char).collect::<Vec<char>>());
        successes += 1;
    }

    successes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process0() -> miette::Result<()> {
        let input = "???.### 1,1,3";
        assert_eq!("1", process(input)?);
        // d = 5 - 3 = 2
        // 7 slots
        // 7 - 2 - 2 = 3
        // 3 choose 3

        let input = "???..#.???.### 1,1,3";
        assert_eq!("6", process(input)?);
        // d = 5 - 4 = 1
        
        // 14 slots
        // 14 - 2 - 2
        // 12 choose 5

        let input = "???..#.???## 1,1,3";
        assert_eq!("4", process(input)?);

        let input = "???##????? 2,3"; // 5 - 2 = 3
        assert_eq!("3", process(input)?);
/*
10 slots
10 - 1 - 2 - 1 = 6
6 choose 2 = 15
how many will have # at 4-5?
    if correspond to first group (2):
        4-5 covers 2 slots: 2 - 2 = 0, means group is found in exactly one spot in reduced record
        (6 - 1 - max(0, (2-4))) = 5
        (6 - 3 - 1) = 2
        2 choose 1 = 2
    if correspond to second group (3):
        3 - 2 = 1, group can be in 2 different spots
        left of slot 4-5 we have 3 slots: 1 group of 2, 2 + 1 padding = 3.
        3 - 3 = 0 dof for left
*/

        let input = "???#??#?.?.???? 5,2";
        assert_eq!("7", process(input)?);
/*
15 - (4 + 1) - 1 = 9
9 choose 2 = 36
how many will have # at 4?
    if correspond to first group (5):
    (9 - min(4, 5) = 9 - 4 = 5
    5 choose 1 = 5
*/

        let input = "?###?????? 3,2,1";
        assert_eq!("3", process(input)?);
        // 10 total
        // 10 slots
        // 10 - 3 - 2 = 5
        // 5 choose 3 = 10
        // ans = 3
        
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!("1", process(input)?);
/*
15 slots
15 - 3 - (2 + 5) = 5
5 choose 4 = 5
# at 2:
    if 1st(1):
        
*/

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        /*
        q3, o1, d3    1,1,3
        5 - 3 = 2 needed

        o1, q2, o2, q2, o3, q1, d2
        q1, d1, q1, d1, q1, d1, q1, d1, q1, d1, q1, d1, q1, d1, q1
        q4, o1, d1, o3, d1, o3
        q4, o1, d6, o1, d5
        q1, d3, q8


        */
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
