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
    match HexValidity::from(number) {
        HexValidity::TooLow => "00".to_string(),
        HexValidity::TooHigh => "FF".to_string(),
        HexValidity::InRange(number) => format!("{:0>2X}", number),
    }
}

#[allow(dead_code)]
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{}{}{}", hex(r), hex(g), hex(b))
}

#[allow(unused_macros)]
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
