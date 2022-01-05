use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut sorterd_word = word.chars().collect::<Vec<_>>();
    sorterd_word.sort_unstable();

    possible_anagrams
        .iter()
        .copied()
        .filter(|w| {
            let w_lower = w.to_lowercase();
            if w_lower == word {
                return false;
            }
            let mut sorted_w = w_lower.chars().collect::<Vec<_>>();
            sorted_w.sort_unstable();
            sorted_w == sorterd_word
        })
        .collect()
}
