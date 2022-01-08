#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    let a = _first_list;
    let b = _second_list;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if a.windows(n).any(|v| v == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if b.windows(m).any(|v| v == a) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
