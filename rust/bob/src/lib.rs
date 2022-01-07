pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.to_uppercase() == x
            && x.matches(char::is_alphabetic).count() > 0
            && x.ends_with("?") =>
        {
            "Calm down, I know what I'm doing!"
        }
        x if x.to_uppercase() == x && x.matches(char::is_alphabetic).count() > 0 => {
            "Whoa, chill out!"
        }
        x if x.ends_with("?") => "Sure.",
        x if x.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
