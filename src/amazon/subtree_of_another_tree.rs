#![allow(dead_code)]

struct Solution {}

use crate::datastructures::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if s.is_none() && t.is_none() {
            true
        } else if s.is_none() || t.is_none() {
            false
        } else {
            let s_node = s.unwrap();
            let t_node = t.unwrap();
            Solution::is_subtree_helper(Some(s_node.clone()), Some(t_node.clone()))
                || Solution::is_subtree(s_node.borrow().left.clone(), Some(t_node.clone()))
                || Solution::is_subtree(s_node.borrow().right.clone(), Some(t_node.clone()))
        }
    }

    fn is_subtree_helper(
        s: Option<Rc<RefCell<TreeNode>>>,
        t: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if s.is_none() && t.is_none() {
            true
        } else if s.is_none() || t.is_none() {
            false
        } else {
            let s_node = s.unwrap();
            let t_node = t.unwrap();

            if s_node.borrow().val != t_node.borrow().val {
                false
            } else {
                Solution::is_subtree_helper(
                    s_node.borrow().left.clone(),
                    t_node.borrow().left.clone(),
                ) && Solution::is_subtree_helper(
                    s_node.borrow().right.clone(),
                    t_node.borrow().right.clone(),
                )
            }
        }
    }
}
