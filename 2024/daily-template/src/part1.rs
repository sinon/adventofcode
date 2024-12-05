#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
