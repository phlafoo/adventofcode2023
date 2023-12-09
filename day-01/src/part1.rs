use crate::custom_error::AocError;

pub fn process(
    input: &str
) -> miette::Result<String, AocError> {
    // Iterate line by line and accumulate result
    let result = input.split('\n').fold(0, |acc, word| {
        let digits = word.matches(char::is_numeric).collect::<Vec<_>>();
        acc + digits.first().unwrap().parse::<i32>().unwrap() * 10
            + digits.last().unwrap().parse::<i32>().unwrap()
    });
    Ok(result.to_string())
}

#[cfg(test)]
mod tests { 
    use super::*;
    
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}