pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    for w in digits.chars().collect::<Vec<char>>().windows(len) {
        let fragment = w
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        output.push(fragment);
    }
    output
}
