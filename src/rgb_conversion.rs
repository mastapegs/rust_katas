use std::collections::HashMap;

enum HexValidity {
    TooHigh,
    TooLow,
    InRange(i32),
}

impl From<i32> for HexValidity {
    fn from(number: i32) -> Self {
        if number < 0 {
            HexValidity::TooLow
        } else if number > 255 {
            HexValidity::TooHigh
        } else {
            HexValidity::InRange(number)
        }
    }
}

fn hex(number: i32) -> String {
    let hex_map: HashMap<i32, String> = HashMap::from([
        (0, "0".to_string()),
        (1, "1".to_string()),
        (2, "2".to_string()),
        (3, "3".to_string()),
        (4, "4".to_string()),
        (5, "5".to_string()),
        (6, "6".to_string()),
        (7, "7".to_string()),
        (8, "8".to_string()),
        (9, "9".to_string()),
        (10, "A".to_string()),
        (11, "B".to_string()),
        (12, "C".to_string()),
        (13, "D".to_string()),
        (14, "E".to_string()),
        (15, "F".to_string()),
    ]);

    match HexValidity::from(number) {
        HexValidity::TooLow => "00".to_string(),
        HexValidity::TooHigh => "FF".to_string(),
        HexValidity::InRange(number) => {
            let mut number_in_progress = number;
            let mut hex_chars: Vec<String> = vec![];

            hex_chars.push(hex_map.get(&(number_in_progress / 16)).unwrap().to_owned());
            number_in_progress = number_in_progress % 16;

            hex_chars.push(hex_map.get(&number_in_progress).unwrap().to_owned());

            hex_chars.join("")
        }
    }
}

#[allow(dead_code)]
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{}{}{}", hex(r), hex(g), hex(b))
}

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("\nGot: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
