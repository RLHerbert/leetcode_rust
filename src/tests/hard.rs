// Legend:
// // pxyz -> problem xyz
// // hxyz -> helper for function xyz

use crate::solutions::Solution;

mod problems {
    use super::Solution;

    #[test]
    #[ignore]
    fn p420_strong_pass() {
        let check_pass = |pass| Solution::strong_password_checker(pass);
        assert_eq!(check_pass("a".into()), 5);
        assert_eq!(check_pass("aA1".into()), 3);
        assert_eq!(check_pass("1337C0d3".into()), 0);
        assert_eq!(check_pass("0123456789Abcdefghijk".into()), 1);
        assert_eq!(check_pass("0123456789abcdefghijk".into()), 2);
        assert_eq!(check_pass("aaaaaaaaaaaaaaaaaaaa".into()), 6);
    }

    #[test]
    #[ignore]
    fn p493_reverse_pairs() {
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
        // Current algo is O(n^2) and at least one input is large,
        // which this test doesn't cover.

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
