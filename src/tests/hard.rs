// Legend:
// // pxyz -> problem xyz
// // hxyz -> helper for function xyz

use crate::solutions::Solution;

mod problems {
    use super::Solution;

    #[test]
    #[ignore]
    pub fn p420_strong_pass() {
        assert_eq!(Solution::strong_password_checker("a".into()), 5);
        assert_eq!(Solution::strong_password_checker("aA1".into()), 3);
        assert_eq!(Solution::strong_password_checker("1337C0d3".into()), 0);
        assert_eq!(
            Solution::strong_password_checker("0123456789Abcdefghijk".into()),
            1
        );
        assert_eq!(
            Solution::strong_password_checker("0123456789abcdefghijk".into()),
            2
        );
        assert_eq!(
            Solution::strong_password_checker("aaaaaaaaaaaaaaaaaaaa".into()),
            6
        );
    }

    #[test]
    #[ignore]
    pub fn p493_reverse_pairs() {
        let example_1 = vec![1, 3, 2, 3, 1];
        let example_2 = vec![2, 4, 3, 5, 1];
        let input_1 = vec![
            2147483647,
            2147483647,
            -2147483647,
            -2147483647,
            -2147483647,
            2147483647,
        ];
        // Algo is O(n^2) and at least one input is large.

        assert_eq!(Solution::reverse_pairs(example_1), 2);
        assert_eq!(Solution::reverse_pairs(example_2), 3);
        assert_eq!(Solution::reverse_pairs(input_1), 9);
    }
}

mod helpers {
    use super::Solution;

    #[test]
    #[ignore]
    fn h420_strong_pass() {
        assert!(!Solution::is_strong_password("a"));
        assert!(!Solution::is_strong_password("aA1"));
        assert!(Solution::is_strong_password("1337C0d3"));
    }
}
