fn square_digits(num: u64) -> u64 {
    let num_str = num.to_string();
    let mut new_digit = String::new();
    for char_digit in num_str.chars() {
        let digit = char_digit.to_digit(10).unwrap();
        new_digit.push_str(&format!("{}", digit.pow(2)));
    }
    new_digit.parse().unwrap()
}


// Functional style
fn square_digits_func(num: u64) -> u64 {
    num
    .to_string()
    .chars()
    .map(|char_digit| char_digit.to_digit(10).unwrap().pow(2).to_string())
    .collect::<String>()
    .parse()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{square_digits, square_digits_func};

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }

    #[test]
    fn test_square_digits_func() {
        assert_eq!(square_digits_func(9119), 811181, "\nFailed with num 9119");
    }
}
