#[cfg(test)]
mod tests {
    #[test]
    fn iterator_examples_filtering() {
        let data = [0, 1, 2, 3, 4, 5];

        let even: Vec<_> = data.iter().filter(|x| (*x % 2) == 0).copied().collect();
        assert_eq!(even, vec![0, 2, 4]);

        let even: Vec<_> = data
            .iter()
            .filter_map(|i| if i % 2 == 0 { Some(*i) } else { None })
            .collect();

        assert_eq!(even, vec![0, 2, 4]);
    }

    #[test]
    fn iterator_examples_folding() {
        let data = [0, 1, 2, 3, 4, 5];

        let sum = data.iter().sum::<i32>();
        assert_eq!(sum, 15);
    }

    #[test]
    fn iterator_examples_flatten() {
        let data = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = data.iter().flatten().copied().collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let result = data.iter().flat_map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let result = data.into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
