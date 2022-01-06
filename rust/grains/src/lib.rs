pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => (),
        _ => panic!("Square must be between 1 and 64"),
    }
    let mut ans: u64 = 1;
    (1..s).for_each(|_| {
        ans = ans * 2;
    });
    ans
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x)) as u64
}
