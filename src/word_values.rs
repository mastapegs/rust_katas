fn single_char_value(char: char) -> i32 {
    (char as u8) as i32 - 96
}

fn single_word_value(word: &str) -> i32 {
    word.split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .map(single_char_value)
        .sum()
}

#[allow(dead_code)]
fn word_value(words: &[&str]) -> Vec<i32> {
    words
        .iter()
        .map(|word| single_word_value(word))
        .enumerate()
        .map(|(index, value)| value * ((index + 1) as i32))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digit() {
        assert_eq!(single_char_value('a'), 1);
        assert_eq!(single_char_value('b'), 2);
        assert_eq!(single_char_value('z'), 26);
    }

    #[test]
    fn basic_tests() {
        assert_eq!(word_value(&["abc", "abc", "abc", "abc"]), [6, 12, 18, 24]);
        assert_eq!(word_value(&["codewars", "abc", "xyz"]), [88, 12, 225]);
    }
}
