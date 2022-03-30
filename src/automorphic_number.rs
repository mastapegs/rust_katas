#[allow(dead_code)]
fn automorphic(n: u64) -> String {
    if n.pow(2).to_string().ends_with(&n.to_string()) {
        "Automorphic".to_string()
    } else {
        "Not!!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(automorphic(1), "Automorphic");
        assert_eq!(automorphic(3), "Not!!");
        assert_eq!(automorphic(6), "Automorphic");
        assert_eq!(automorphic(9), "Not!!");
        assert_eq!(automorphic(25), "Automorphic");
        assert_eq!(automorphic(53), "Not!!");
        assert_eq!(automorphic(76), "Automorphic");
        assert_eq!(automorphic(95), "Not!!");
        assert_eq!(automorphic(625), "Automorphic");
        assert_eq!(automorphic(225), "Not!!");
    }
}
