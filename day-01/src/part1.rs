use crate::custom_error::AocError;

pub fn process(
    input: &str
) -> miette::Result<String, AocError> {
    // Ok("".to_string())
    Ok(input.to_string())
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert!("", process(input)?);
        Ok(())
    }
}