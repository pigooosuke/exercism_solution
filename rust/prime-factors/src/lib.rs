pub fn factors(n: u64) -> Vec<u64> {
    let mut divided = n;
    let mut ans = vec![];
    for i in 2..=((n as f64).sqrt()) as u64 {
        while divided % i == 0 {
            ans.push(i);
            divided = divided / i;
        }
    }
    if divided != 1 {
        ans.push(divided);
    }
    ans
}
