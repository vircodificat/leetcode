use std::collections::{HashSet, VecDeque};

pub fn length_of_longest_substring(mut s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut result: i32 = 1;

    let mut h = HashSet::new();
    let mut q = VecDeque::new();
    let first = s.pop().unwrap();

    q.push_back(first);
    h.insert(first);

    while let Some(ch) = s.pop() {
        while h.contains(&ch) {
            let remove_ch = q.pop_front().unwrap();
            h.remove(&remove_ch);
        }

        h.insert(ch);
        q.push_back(ch);

        result = result.max(q.len().try_into().unwrap());
    }

    result
}

fn main() {
    assert_eq!(length_of_longest_substring("".to_string()), 0);
    assert_eq!(length_of_longest_substring("a".to_string()), 1);
    assert_eq!(length_of_longest_substring("ab".to_string()), 2);
    assert_eq!(length_of_longest_substring("abb".to_string()), 2);
    assert_eq!(length_of_longest_substring("bab".to_string()), 2);
    assert_eq!(length_of_longest_substring("abcba".to_string()), 3);
    println!("DONE");
}
