use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .as_bytes()
        .split(|&c| c == b',')
        .map(hash)
        .sum::<u32>()
        .to_string()
    )
}

fn hash(step: &[u8]) -> u32 {
    step.iter().fold(0, |acc, &c| (acc + c as u32) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
