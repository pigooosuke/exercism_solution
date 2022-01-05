const RADIX: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let digit = num.to_string().len() as u32;
    let result = num.to_string().chars().fold(0, |acc, v| {
        acc + ((v.to_digit(RADIX).unwrap() as u64).pow(digit))
    });
    // dbg!(num.to_string().chars().collect::<Vec<_>>());
    // dbg!(num, result, digit);
    num as u64 == result
}
