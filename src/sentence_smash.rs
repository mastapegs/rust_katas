#[allow(dead_code)]
fn smash(words: &'static [&str]) -> String {
    words.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(smash(&[]), "");
        assert_eq!(smash(&["hello"]), "hello");
        assert_eq!(smash(&["hello", "world"]), "hello world");
        assert_eq!(smash(&["hello", "amazing", "world"]), "hello amazing world");
        assert_eq!(
            smash(&["this", "is", "a", "really", "long", "sentence"]),
            "this is a really long sentence"
        );
    }
}
