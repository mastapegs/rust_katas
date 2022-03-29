// Assume "#" is like a backspace in string. This means that string "a#bc#d" actually is "bd"

// Your task is to process a string with "#" symbols.

#[allow(dead_code)]
fn clean_string(s: &str) -> String {
    match s.contains("#") {
        false => s.to_string(),
        true => {
            let pound_location = s.find("#").unwrap();
            let mut characters = s.chars().collect::<Vec<char>>();

            match pound_location {
                0 => {
                    characters.remove(0);
                }
                _ => {
                    characters.remove(pound_location);
                    characters.remove(pound_location - 1);
                }
            }
            clean_string(
                &characters
                    .iter()
                    .map(|char| char.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
            )
        }
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
