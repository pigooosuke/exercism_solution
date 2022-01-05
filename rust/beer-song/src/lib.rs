pub fn beer_bottles(n: u32) -> String {
    match n {
        2..=u32::MAX => format!("{} bottles", n),
        1 => "1 bottle".to_string(),
        0 => "No more bottles".to_string(),
    }
}

pub fn verse(n: u32) -> String {
    let next_n = match n {
        1..=u32::MAX => n - 1,
        0 => 99,
    };
    let bottles = beer_bottles(n);
    let next_bottles = beer_bottles(next_n);
    let take_down = match n {
        2..=u32::MAX => "Take one down and pass it around,".to_string(),
        1 => "Take it down and pass it around,".to_string(),
        0 => "Go to the store and buy some more,".to_string(),
    };
    let lyrics = format!(
        "{bottles_first} of beer on the wall, {bottles_second} of beer.\n\
    {take_down} {next_bottles} of beer on the wall.\n",
        bottles_first = bottles,
        bottles_second = bottles.to_lowercase(),
        next_bottles = next_bottles.to_lowercase(),
        take_down = take_down
    );
    lyrics
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<_>>()
        .join("\n")
}
