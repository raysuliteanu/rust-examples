extern crate core;

use std::collections::HashMap;

pub mod crypto;
pub mod iterators;
pub mod math;
pub mod sort;
pub mod state;
pub mod token;
pub mod trie;

/// Dynamic programming (using memoization) example
/// for an NxM matrix how many paths are there from top left to bottom right
/// if you can "move" only right or down at each step?
/// In this solution, we pre-compute (memoize) the answer, then just look it up
pub fn paths(n: u32, m: u32) -> u64 {
    debug_assert_ne!(n, 0);
    debug_assert_ne!(m, 0);

    let mut memo: HashMap<(u32, u32), u64> = HashMap::new();

    for i in 1..=n {
        memo.insert((i, 1), 1);
    }

    for i in 1..=m {
        memo.insert((1, i), 1);
    }

    for i in 2..=n {
        for j in 2..=m {
            memo.insert((i, j), memo[&(i - 1, j)] + memo[&(i, j - 1)]);
        }
    }

    memo[&(n, m)]
}

/// given two arrays that are sorted with distinct elements (i.e. no dupes in an array)
/// find how many elements in common between the two.
/// This impl runs in O(min(v1 size, v2 size)) since it iterates over the array elements
/// at most the length of the smaller of the two
fn common_elements(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx1 < v1.len() && idx2 < v2.len() {
        if v1[idx1] == v2[idx2] {
            count += 1;
            idx1 += 1;
            idx2 += 1;
        } else if v1[idx1] < v2[idx2] {
            idx1 += 1;
        } else if v1[idx1] > v2[idx2] {
            idx2 += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_elements() {
        let in1 = vec![1, 2, 4, 5, 8, 18, 21, 22];
        let in2 = vec![2, 3, 8, 9, 11, 18, 19, 21, 29];
        assert_eq!(common_elements(in1, in2), 4);

        let in1 = vec![1];
        let in2 = vec![];
        assert_eq!(common_elements(in1, in2), 0);

        let in1 = vec![];
        let in2 = vec![1];
        assert_eq!(common_elements(in1, in2), 0);
    }

    #[test]
    fn base_case_grid() {
        assert_eq!(paths(1, 1), 1);
        assert_eq!(paths(10, 1), 1);
        assert_eq!(paths(1, 10), 1);
    }

    #[test]
    fn non_trivial_grid() {
        assert_eq!(paths(18, 6), 26334);
        assert_eq!(paths(75, 19), 5873182941643167150);
    }
}
