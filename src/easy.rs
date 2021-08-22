// 1. Two Sum
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let elt_idx: HashMap<_, _> = (&nums)
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, elt)| (elt, idx))
        .collect();

    let mut out = vec![];

    for &elt in &nums {
        if let Some(&idx) = elt_idx.get(&(target - elt)) {
            if *elt_idx.get(&elt).unwrap() != idx {
                out.push(*elt_idx.get(&elt).unwrap() as i32);
                out.push(idx as i32);
                break;
            }
        }
    }

    if out.is_empty() {
        out.push(*elt_idx.get(&(target / 2)).unwrap() as i32);
        out.push(
            nums.iter()
                .cloned()
                .enumerate()
                .find(|&(_, elt)| elt == target / 2)
                .map(|(idx, _)| idx)
                .unwrap() as i32,
        )
    }

    out
}

// 1920. Build Array from Permutation
#[allow(dead_code)]
pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    (0..nums.len())
        .map(|elt| nums[nums[elt as usize] as usize] as i32)
        .collect()
}
