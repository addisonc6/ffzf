pub fn char_vec(word: &str, case_sensitive: bool) -> Vec<char> {
    if case_sensitive {
        return word.chars().collect::<Vec<_>>();
    }
    return word.to_lowercase().chars().collect::<Vec<_>>();
}