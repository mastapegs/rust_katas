#[allow(dead_code)]
fn adjacent_elements_product(xs: &[i32]) -> i32 {
    xs.windows(2)
        .map(|pair_of_numbers| pair_of_numbers.into_iter().fold(1, |a, b| a * b))
        .max()
        .unwrap()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_numbers() {
        assert_eq!(adjacent_elements_product(&[5, 8]), 40);
        assert_eq!(adjacent_elements_product(&[1, 2, 3]), 6);
        assert_eq!(adjacent_elements_product(&[1, 5, 10, 9]), 90);
        assert_eq!(adjacent_elements_product(&[4, 12, 3, 1, 5]), 48);
        assert_eq!(adjacent_elements_product(&[5, 1, 2, 3, 1, 4]), 6);
    }

    #[test]
    fn mixed_values() {
        assert_eq!(adjacent_elements_product(&[3, 6, -2, -5, 7, 3]), 21);
        assert_eq!(adjacent_elements_product(&[9, 5, 10, 2, 24, -1, -48]), 50);
        assert_eq!(adjacent_elements_product(&[5, 6, -4, 2, 3, 2, -23]), 30);
        assert_eq!(
            adjacent_elements_product(&[-23, 4, -5, 99, -27, 329, -2, 7, -921]),
            -14
        );
        assert_eq!(adjacent_elements_product(&[5, 1, 2, 3, 1, 4]), 6);
    }

    #[test]
    fn containing_zeroes() {
        assert_eq!(adjacent_elements_product(&[1, 0, 1, 0, 1000]), 0);
        assert_eq!(adjacent_elements_product(&[1, 2, 3, 0]), 6);
    }
}
