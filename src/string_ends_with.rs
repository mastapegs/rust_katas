#[allow(dead_code)]
pub fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Rust test example:
    // TODO: replace with your own tests (TDD), these are just how-to examples.
    // See: https://doc.rust-lang.org/book/testing.html

    #[test]
    fn returns_expected() {
        assert!(solution("abc", "c"));
        assert!(!solution("strawberry", "banana"));
    }
}
