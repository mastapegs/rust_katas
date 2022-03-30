use std::cmp::Ordering;

#[allow(dead_code)]
pub fn power_of_two(x: u64) -> bool {
    match x {
        0 => false,
        1 => true,
        x => {
            let mut exponent = 1;
            loop {
                match x.cmp(&2_u64.pow(exponent)) {
                    Ordering::Equal => break true,
                    Ordering::Greater => exponent += 1,
                    Ordering::Less => break false,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert!(!power_of_two(0));
        assert!(power_of_two(2));
        assert!(!power_of_two(5));
        assert!(!power_of_two(6));
        assert!(power_of_two(8));
        assert!(power_of_two(1024));
        assert!(power_of_two(4096));
        assert!(!power_of_two(8191));
    }
}
