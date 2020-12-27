#![allow(dead_code, unused_imports)]

struct Solution {}

use crate::datastructures::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        let mut res: f64 = 0.0;

        Solution::dfs(root, &mut res);

        res
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut f64) -> (i32, f64) {
        if root.is_none() {
            (0, 0.0)
        } else {
            let node = root.unwrap();
            let (left_count, left_rs) = Solution::dfs(node.borrow().left.clone(), res);
            let (right_count, right_rs) = Solution::dfs(node.borrow().right.clone(), res);

            let count = left_count + right_count + 1;
            let rs = left_rs + right_rs + node.borrow().val as f64;

            if (rs / count as f64) > *res {
                *res = rs / count as f64;
            }

            (count, rs)
        }
    }
}
