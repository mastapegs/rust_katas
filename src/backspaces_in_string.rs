// Assume "#" is like a backspace in string. This means that string "a#bc#d" actually is "bd"

// Your task is to process a string with "#" symbols.

#[allow(dead_code)]
fn clean_string(s: &str) -> String {
    if s.contains('#') {
        let mut characters = s.chars().collect::<Vec<char>>();

        let pound_location = s.find('#').unwrap();
        if pound_location == 0 {
            characters.remove(0);
        } else {
            characters.remove(pound_location);
            characters.remove(pound_location - 1);
        }

        clean_string(&characters.iter().map(char::to_string).collect::<String>())
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(clean_string("abc#d##c"), "ac");
        assert_eq!(clean_string("abc####d##c#"), "");
        assert_eq!(clean_string("#######"), "");
        assert_eq!(clean_string(""), "");
    }
}
