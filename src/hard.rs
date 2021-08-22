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

    let password: Vec<char> = password.chars().collect();
    0
}

// #[allow(dead_code)]
// // fn edit_distance_to_strong_pass(password: String) -> i32 {}

#[allow(dead_code)]
fn is_strong_password(password: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn strong_passwords() {
        assert!(!is_strong_password("a"));
        assert!(!is_strong_password("aA1"));
        assert!(is_strong_password("1337C0d3"));
    }

    #[test]
    fn correct_edit_distances() {
        assert_eq!(strong_password_checker("a".into()), 5);
        assert_eq!(strong_password_checker("aA1".into()), 3);
        assert_eq!(strong_password_checker("1337C0d3".into()), 0);
        assert_eq!(strong_password_checker("0123456789Abcdefghijk".into()), 1);
        assert_eq!(strong_password_checker("0123456789abcdefghijk".into()), 2);
        assert_eq!(strong_password_checker("aaaaaaaaaaaaaaaaaaaa".into()), 6);
        // assert_eq!(strong_password_checker("aaaaaaaaaaaaaaaaaaaa1".into()), 6);
        // assert_eq!(strong_password_checker("1337C0d3".into()), 0);
        // assert_eq!(strong_password_checker("1337C0d3".into()), 0);
    }

    #[test]
    fn reverse_pairs_test() {
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

        assert_eq!(reverse_pairs(example_1), 2);
        assert_eq!(reverse_pairs(example_2), 3);
        assert_eq!(reverse_pairs(input_1), 9);
    }
}

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
