pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for s in string.chars() {
        match s {
            '[' | '{' | '(' => stack.push(s),
            ']' => {
                if Some('[') != stack.pop() {
                    return false;
                }
            }
            '}' => {
                if Some('{') != stack.pop() {
                    return false;
                }
            }
            ')' => {
                if Some('(') != stack.pop() {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
