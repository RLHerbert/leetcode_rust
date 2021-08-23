use super::Solution;

// 420. Strong Password Checker
impl Solution {
    #[allow(dead_code)]
    // 420. Strong Password Checker
    pub fn strong_password_checker(password: String) -> i32 {
        // let _ = (password.chars().count(), ());
        // // Vec of chars, mutable? Then perform the edits, we only care about distance.
        // // Then calculate edit distance?
        // let length = password.chars().count();
        // let length_ok = (6..=20).contains(&length);
        // let contains_lower = password.chars().any(|ch| ch.is_ascii_lowercase());
        // let contains_upper = password.chars().any(|ch| ch.is_ascii_uppercase());
        // let contains_number = password.chars().any(|ch| ch.is_numeric());
        // let max_two_repeating = password
        //     .chars()
        //     .collect::<Vec<char>>()
        //     .windows(3)
        //     .all(|chs| chs[0] != chs[1] || chs[1] != chs[2]);

        let _password: Vec<char> = password.chars().collect();
        0
    }

    // Helpers
    #[allow(dead_code)]
    pub fn is_strong_password(password: &str) -> bool {
        let password = String::from(password);
        let length = password.chars().count();
        let length_ok = (6..=20).contains(&length);
        let contains_lower = password.chars().any(|ch| ch.is_ascii_lowercase());
        let contains_upper = password.chars().any(|ch| ch.is_ascii_uppercase());
        let contains_number = password.chars().any(|ch| ch.is_numeric());
        let max_two_repeating = password
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .all(|chs| chs[0] != chs[1] || chs[1] != chs[2]); // DeMorgen's this shit

        length_ok && contains_lower && contains_upper && contains_number && max_two_repeating
    }
}

// 493. Reverse Pairs
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .map(|i| {
                ((i + 1)..nums.len())
                    .filter(|&j| match nums[j].checked_mul(2) {
                        Some(n) => nums[i] > n,
                        None => nums[j] < 0,
                    })
                    .count()
            })
            .sum::<usize>() as i32
    }
}
