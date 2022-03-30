#[allow(dead_code)]
fn multiple_3_5_sum(below: u32) -> u32 {
    (1..below)
        .into_iter()
        .filter(|number| number % 3 == 0 || number % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(23, multiple_3_5_sum(10));
        assert_eq!(233_168, multiple_3_5_sum(1000));
    }
}
