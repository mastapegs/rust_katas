fn factorial(number: u64) -> u64 {
    (1..=number).product()
}

#[allow(dead_code)]
fn strong(n: u64) -> String {
    match n
        == n.to_string()
            .chars()
            .map(|char| factorial(char.to_string().parse::<u64>().unwrap()))
            .sum()
    {
        true => "STRONG!!!!".to_string(),
        false => "Not Strong !!".to_string(),
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(6, factorial(3));
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn test_strong() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");

        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");

        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");

        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");

        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");

        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");
    }
}
