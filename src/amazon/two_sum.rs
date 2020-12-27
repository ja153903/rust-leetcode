#![allow(dead_code)]

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i, val) in nums.iter().enumerate() {
            if seen.contains_key(&(target - val)) {
                if let Some(&j) = seen.get(&(target - val)) {
                    return vec![j as i32, i as i32];
                }
            }

            seen.insert(val, i);
        }

        vec![]
    }
}
