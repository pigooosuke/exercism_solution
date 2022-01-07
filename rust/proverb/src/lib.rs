pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::new();
    for w in list.windows(2) {
        let (prev, next) = (w[0], w[1]);
        output.push_str(&format!("For want of a {} the {} was lost.\n", prev, next));
    }
    if output.len() > 0 {
        output.push_str(&format!("And all for the want of a {}.", list[0]));
    } else if list.len() == 1 {
        output.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    output
}
