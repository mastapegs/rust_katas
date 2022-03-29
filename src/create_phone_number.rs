fn numbers_to_string(numbers: &[u8]) -> String {
    numbers.iter().map(|number| number.to_string()).collect()
}

#[allow(dead_code)]
fn create_phone_number(numbers: &[u8]) -> String {
    format!(
        "({}) {}-{}",
        numbers_to_string(&numbers[0..3]),
        numbers_to_string(&numbers[3..6]),
        numbers_to_string(&numbers[6..10])
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_conversion() {
        assert_eq!(numbers_to_string(&[1, 2, 3]), "123");
    }

    #[test]
    fn returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
