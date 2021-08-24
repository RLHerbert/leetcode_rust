use super::Solution;

// 1. Two Sum
impl Solution {
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

        out.sort_unstable(); // Not necessary. Just to make testing easier.

        out
    }
}

// 21. Merge Two Sorted Lists
impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(l1: Option<Node>, l2: Option<Node>) -> Option<Node> {
        match (l1, l2) {
            (None, None) => None,
            (None, Some(node)) => Some(Node::new(ListNode {
                val: node.val,
                next: Solution::merge_two_lists(None, node.next),
            })),
            (Some(node), None) => Some(Node::new(ListNode {
                val: node.val,
                next: Solution::merge_two_lists(node.next, None),
            })),
            (Some(node1), Some(node2)) => Some(Node::new(match node1.val < node2.val {
                true => ListNode {
                    val: node1.val,
                    next: Solution::merge_two_lists(node1.next, Some(node2)),
                },
                false => ListNode {
                    val: node2.val,
                    next: Solution::merge_two_lists(Some(node1), node2.next),
                },
            })),
        }
    }
}

type Node = Box<ListNode>;
// type MaybeNode = Option<Node>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 1920. Build Array from Permutation
impl Solution {
    #[allow(dead_code)]
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .map(|elt| nums[nums[elt as usize] as usize] as i32)
            .collect()
    }
}
