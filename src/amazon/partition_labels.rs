#![allow(dead_code)]

struct Solution {}

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();

        s.chars().enumerate().for_each(|(i, c)| {
            map.insert(c.to_string(), i);
        });

        let mut start: usize = 0;
        let mut last: usize = 0;

        for i in 0..s.len() {
            last = cmp::max(last, *map.get(&s[i..i + 1]).unwrap());

            if i == last {
                result.push((last - start + 1) as i32);
                start = last + 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_test() {
        let s = String::from("ababcbacadefegdehijhklij");

        let expected: Vec<i32> = vec![9, 7, 8];

        assert_eq!(Solution::partition_labels(s), expected);
    }
}
