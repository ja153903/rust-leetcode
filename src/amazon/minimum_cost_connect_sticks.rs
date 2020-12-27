#![allow(dead_code)]

struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut minheap = BinaryHeap::new();
        let mut cost: i32 = 0;

        sticks
            .iter()
            .for_each(|stick| minheap.push(Reverse(*stick)));

        while minheap.len() > 1 {
            let x = minheap.pop().unwrap().0;
            let y = minheap.pop().unwrap().0;

            cost += x + y;

            minheap.push(Reverse(x + y));
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_test() {
        let sticks = vec![2, 4, 3];
        let expected = 14;

        assert_eq!(Solution::connect_sticks(sticks), expected);
    }
}
