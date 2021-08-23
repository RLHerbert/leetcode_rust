// Legend:
// // pxyz -> problem xyz
// // hxyz -> helper for function xyz
use crate::solutions::Solution;
mod problems {
    use super::Solution;
    #[test]
    #[ignore]
    fn p1_two_sum() {
        let (nums, target) = (vec![2, 7, 11, 15], 9);
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);

        let (nums, target) = (vec![3, 2, 4], 6);
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);

        let (nums, target) = (vec![3, 3], 6);
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    #[ignore]
    fn p1920_array_from_perm() {
        let nums = vec![0, 2, 1, 5, 3, 4];
        assert_eq!(Solution::build_array(nums), vec![0, 1, 2, 4, 5, 3]);

        let nums = vec![5, 0, 1, 2, 3, 4];
        assert_eq!(Solution::build_array(nums), vec![4, 5, 0, 1, 2, 3]);
    }
}

// mod helpers {}
