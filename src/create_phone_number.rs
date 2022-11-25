
fn create_phone_number(numbers: &[u8]) -> String {
    let phone_number: String = numbers.iter().map(|d| d.to_string()).collect();
    format!(
        "({}) {}-{}",
        &phone_number[..3],
        &phone_number[3..6],
        &phone_number[6..],
    )    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
    }
}