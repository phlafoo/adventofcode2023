use crate::custom_error::AocError;

pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    todo!("day 01 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

/*
???.### 1,1,3
 1 arrangement

.??..??...?##. 1,1,3
 16384 arrangements
4 * (4 + 4) * 8 * 8 * 8 = 16384

?#?#?#?#?#?#?#? 1,3,1,6
 1 arrangement

????.#...#... 4,1,1
 16 arrangements
 1 * 2 * 2 * 2 * 2 = 16

????.######..#####. 1,6,5
2500 arrangements
4 * 5 * 5 * 5 * 5 =  2500

?###???????? 3,2,1
506250 arrangements
10 * (225^2)
10 * 15 * 15 * 15 * 15 = 506250


*/