pub fn char_vec(word: &str, case_sensitive: bool, remove_whitespace: bool) -> Vec<char> {
    if !remove_whitespace {
        if case_sensitive {
            return word.chars().collect::<Vec<_>>();
        }
        return word.to_lowercase().chars().collect::<Vec<_>>();
    }
    else {
        if case_sensitive {
            return word.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
        }
        return word.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
    }
}
