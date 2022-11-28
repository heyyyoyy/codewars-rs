fn disemvowel(s: &str) -> String {
    s
    .chars()
    .filter(|&ch| {
        !"aeiou".contains(ch.to_ascii_lowercase())
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
