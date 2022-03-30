#[allow(dead_code)]
fn odd_or_even(numbers: &[i32]) -> String {
    match numbers.iter().sum::<i32>() % 2 {
        0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        assert_eq!(odd_or_even(&[]), "even");
    }
    #[test]
    fn test_single_even() {
        assert_eq!(odd_or_even(&[0]), "even");
    }
    #[test]
    fn test_single_odd() {
        assert_eq!(odd_or_even(&[1]), "odd");
    }
    #[test]
    fn test_even() {
        assert_eq!(odd_or_even(&[0, 1, 5]), "even");
    }
    #[test]
    fn test_odd() {
        assert_eq!(odd_or_even(&[0, 1, 4]), "odd");
    }
    #[test]
    fn test_negative_even() {
        assert_eq!(odd_or_even(&[0, -1, -5]), "even");
    }
    #[test]
    fn test_negative_odd() {
        assert_eq!(odd_or_even(&[0, -1, 2]), "odd");
    }
}
