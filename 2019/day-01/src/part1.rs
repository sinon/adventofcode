#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    Ok(_input
        .lines()
        .map(|s| {
            let i = s.parse::<f32>().unwrap();
            ((i / 3.0).floor() - 2.0) as i32
        })
        .sum())
}

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
