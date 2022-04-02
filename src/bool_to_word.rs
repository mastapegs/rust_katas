#[allow(dead_code)]
fn bool_to_word(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::bool_to_word;

    #[test]
    fn example_tests() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }
}
