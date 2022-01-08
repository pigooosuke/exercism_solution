use std::char::from_digit;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len())
        .map(|y| {
            (0..minefield[y].len())
                .map(|x| annotate_square(x, y, minefield))
                .collect()
        })
        .collect()
}

fn annotate_square(x: usize, y: usize, minefield: &[&str]) -> char {
    match &minefield[y].chars().nth(x).unwrap() {
        '*' => '*',
        ' ' => match count_adjacent_mines(x, y, minefield) {
            0 => ' ',
            n => from_digit(n as u32, 10).unwrap(),
        },
        _ => panic!(),
    }
}

fn count_adjacent_mines(x: usize, y: usize, minefield: &[&str]) -> usize {
    minefield
        .iter()
        .skip(if y == 0 { 0 } else { y - 1 })
        .take(if y == 0 { 2 } else { 3 })
        .map(|row| {
            row.chars()
                .skip(if x == 0 { 0 } else { x - 1 })
                .take(if x == 0 { 2 } else { 3 })
                .filter(|c| *c == '*')
                .count()
        })
        .sum()
}
