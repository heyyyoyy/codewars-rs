fn likes(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".to_string(),
        [first] => format!("{} likes this", first),
        [first, second] => format!("{} and {} like this", first, second),
        [first, second, third] => {
            format!("{}, {} and {} like this", first, second, third)
        },
        [first, second, others @ ..] => {
            format!("{}, {} and {} others like this", first, second, others.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}