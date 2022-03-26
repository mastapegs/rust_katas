#[allow(dead_code)]
pub fn sum_even_fib_under(below: u32) -> u32 {
    let mut fibs: Vec<u32> = vec![1, 2];
    loop {
        let next_fib = fibs[fibs.len() - 1] + fibs[fibs.len() - 2];
        if next_fib > below {
            break;
        }
        fibs.push(next_fib);
    }
    fibs.into_iter().filter(|number| number % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(10, sum_even_fib_under(20));
        assert_eq!(4_613_732, sum_even_fib_under(4_000_000));
    }
}
