pub fn collatz(n: u64) -> Option<u64> {
    let mut step = 0;
    let mut n = n;
    if n == 0 {
        return None;
    }
    while n != 1 {
        if n % 2 == 0 {
            n = n.checked_div(2)?;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        step += 1;
    }
    Some(step)
}
