pub fn number(user_number: &str) -> Option<String> {
    user_number
        .chars()
        .filter(char::is_ascii_digit)
        .try_fold(String::new(), |acc, x| match (acc.len(), x) {
            (0, '1') => Some(acc),
            (0, '0') | (3, '0'..='1') | (10.., _) => None,
            _ => Some(acc + &x.to_string()),
        })
        .filter(|x| x.len() == 10)
}
