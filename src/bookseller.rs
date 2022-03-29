#[allow(dead_code)]
fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    match (list_art.is_empty(), list_cat.is_empty()) {
        (false, false) => list_cat
            .iter()
            .map(|category| {
                format!(
                    "({category} : {count})",
                    count = {
                        list_art
                            .iter()
                            .filter(|book| book.starts_with(category))
                            .map(|book| {
                                book.split_whitespace().collect::<Vec<&str>>()[1]
                                    .parse::<i32>()
                                    .unwrap()
                            })
                            .sum::<i32>()
                    }
                )
            })
            .collect::<Vec<String>>()
            .join(" - "),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }

    #[test]
    fn advanced_tests() {
        let b = vec![];
        let c = vec!["B", "R", "D", "X"];
        dotest(b, c, "")
    }
}
