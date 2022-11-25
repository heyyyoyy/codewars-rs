use std::collections::HashMap;


fn count_duplicates(text: &str) -> u32 {
    let mut pair: HashMap<char, u32> = HashMap::new();
    
    for ch in text.to_lowercase().chars() {
        pair.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }

    pair
    .values()
    .filter(|&&value| value > 1)
    .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
