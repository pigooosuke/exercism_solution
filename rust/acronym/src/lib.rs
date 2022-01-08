pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| {
            if s.to_uppercase() == s {
                s.chars().nth(0).unwrap().to_string()
            } else {
                s.to_string()
            }
        })
        .map(|s| {
            s.chars()
                .enumerate()
                .filter(|(i, c)| *i == 0 as usize || c.is_uppercase())
                .map(|(_, c)| c.to_uppercase().to_string())
                .collect::<String>()
        })
        .collect()
}
