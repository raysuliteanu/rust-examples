pub mod trie;

use std::collections::HashMap;

pub fn paths(n: u32, m: u32) -> u64 {
    debug_assert_ne!(n, 0);
    debug_assert_ne!(m, 0);

    let mut memo : HashMap<(u32, u32), u64> = HashMap::new();

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

#[cfg(test)]
mod tests {
    use super::*;

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
