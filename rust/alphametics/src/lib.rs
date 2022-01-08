use itertools::Itertools;
use std::collections::HashMap;
use std::iter;
use std::str::CharIndices;

// from: https://exercism.org/tracks/rust/exercises/alphametics/solutions/ralli
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let problem = Problem::parse(input)?;
    let chars = problem.all_chars();
    if chars.len() > 10 {
        return None;
    }
    let values: Vec<u8> = (0..=9).collect();
    let mut assignment = [0u8; 26];
    let word_starts = problem.word_starts();
    for perm in values.iter().permutations(values.len()) {
        for (i, &v) in perm.iter().take(chars.len()).enumerate() {
            assignment[((chars[i] as u8) - ('A' as u8)) as usize] = *v;
        }
        if !word_starts.iter().any(|c| assignment[*c] == 0) {
            if problem.is_assignment_valid(&assignment) {
                let mut result = HashMap::new();
                for c in chars {
                    result.insert(c, assignment[(c as u8 - 'A' as u8) as usize]);
                }
                return Some(result);
            }
        }
    }
    None
}
#[derive(Debug, PartialEq)]
struct Problem<'a> {
    words: Vec<&'a str>,
    result: &'a str,
}
impl<'a> Problem<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let mut parts: Vec<&'a str> = Splitter::new(input).collect();
        let last = parts.pop()?;
        Some(Problem {
            words: parts,
            result: last,
        })
    }
    fn all_chars(&self) -> Vec<char> {
        let mut chars: Vec<char> = self
            .words
            .iter()
            .chain(iter::once(&self.result))
            .flat_map(|&word| word.chars())
            .collect();
        chars.sort();
        chars.dedup();
        chars
    }
    fn word_starts(&self) -> Vec<usize> {
        let mut chars: Vec<usize> = self
            .words
            .iter()
            .chain(iter::once(&self.result))
            .map(|&word| word.chars().next().unwrap())
            .map(|c| c as usize - 'A' as usize)
            .collect();
        chars.sort();
        chars.dedup();
        chars
    }
    fn is_assignment_valid(&self, assignment: &[u8]) -> bool {
        let mut words_sum = 0;
        for &word in self.words.iter() {
            words_sum += word_to_number(word, assignment);
        }
        words_sum == word_to_number(self.result, assignment)
    }
}
fn word_to_number(word: &str, assignment: &[u8]) -> u64 {
    let mut result: u64 = 0;
    for c in word.chars() {
        let bla = assignment[((c as u8) - ('A' as u8)) as usize];
        result = result * 10 + bla as u64;
    }
    result
}
struct Splitter<'a> {
    input: &'a str,
    it: CharIndices<'a>,
}
impl<'a> Splitter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            it: input.char_indices(),
        }
    }
}
impl<'a> Iterator for Splitter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start_idx, _)) = self.it.find(|(_, c)| c.is_alphabetic()) {
            if let Some((end_idx, _)) = self.it.find(|(_, c)| !c.is_alphabetic()) {
                Some(&self.input[start_idx..end_idx])
            } else {
                Some(&self.input[start_idx..])
            }
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn splitter_test() {
        let splitter = Splitter::new("SEND + MORE + MONEY");
        let expected = vec!["SEND", "MORE", "MONEY"];
        let result = splitter.collect::<Vec<_>>();
        assert_eq!(result, expected);
    }
    #[test]
    fn parse_problem_test() {
        let input = "SEND + MORE + MONEY";
        let problem = Problem::parse(input);
        let expected = Some(Problem {
            words: vec!["SEND", "MORE"],
            result: "MONEY",
        });
        assert_eq!(problem, expected);
    }
    #[test]
    fn all_chars_test() {
        let input = "SEND + MORE + MONEY";
        let problem = Problem::parse(input).unwrap();
        let expected = vec!['D', 'E', 'M', 'N', 'O', 'R', 'S', 'Y'];
        let result = problem.all_chars();
        assert_eq!(result, expected);
    }
    #[test]
    fn word_starts_test() {
        let input = "SEND + MORE + MONEY";
        let problem = Problem::parse(input).unwrap();
        let expected = vec![12, 18];
        let result = problem.word_starts();
        assert_eq!(result, expected);
    }
}
