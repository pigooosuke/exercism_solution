pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| {
            for factor in factors {
                if *factor == 0 {
                    return false;
                }
                if x % factor == 0 {
                    return true;
                }
            }
            return false;
        })
        .sum()
}
