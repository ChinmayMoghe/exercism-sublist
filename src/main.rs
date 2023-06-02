#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(small_list: &[T], large_list: &[T]) -> bool {
    large_list
        .windows(small_list.len())
        .any(|v| v == small_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if m > n => {
            if is_sublist(_second_list, _first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if m < n => {
            if is_sublist(_first_list, _second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn main() {
    sublist(&[1, 2, 4], &[1, 2, 4, 5]);
}
