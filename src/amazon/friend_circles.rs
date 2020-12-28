#![allow(dead_code)]

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        // Create parent structuring
        // We will initialize the parents as themselves
        let mut parent: Vec<i32> = vec![0; m.len()];

        for i in 0..m.len() {
            parent[i] = i as i32;
        }

        for i in 0..m.len() {
            for j in 0..m.len() {
                if i != j && m[i][j] == 1 {
                    Solution::union(&mut parent, i as i32, j as i32);
                }
            }
        }

        // Once we do the initial union, then we have to run
        // find again to get the final result
        let mut result = HashSet::new();

        for i in 0..m.len() {
            result.insert(Solution::find(&mut parent, i as i32));
        }

        result.len() as i32
    }

    // TODO: Implement find algorithm
    // The idea of find is to find the parent of a
    // given node that we're passing in
    fn find(parent: &mut Vec<i32>, node: i32) -> i32 {
        let mut node = node;
        while parent[node as usize] != node {
            // Set the parent of the current node to be the parent of its parent
            parent[node as usize] = parent[parent[node as usize] as usize];
            node = parent[node as usize];
        }

        parent[node as usize]
    }

    // TODO: Implement union algorithm
    fn union(parent: &mut Vec<i32>, v1: i32, v2: i32) -> bool {
        let v1_parent = Solution::find(parent, v1);
        let v2_parent = Solution::find(parent, v2);

        if v1_parent == v2_parent {
            return false;
        }

        parent[v1_parent as usize] = v2_parent;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_test() {
        let m: Vec<Vec<i32>> = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];

        assert_eq!(Solution::find_circle_num(m), 2);
    }

    #[test]
    pub fn should_pass_test2() {
        let m: Vec<Vec<i32>> = vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]];

        assert_eq!(Solution::find_circle_num(m), 1);
    }
}

// Create an adjancency list to represent out graph
// Then we want to implement some sort of union-find algorithm
// The reason we want to use a union-find algorithm is that
// the solution to the number of friend circles basically
// depends on finding the number of connected components
// Once we find the number of connected components, then
// we will know the number of friend groups
