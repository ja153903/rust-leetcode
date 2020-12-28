#![allow(dead_code)]

struct Solution {}

use crate::datastructures::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            None
        } else {
            let node = root.unwrap();
            let p_node = p.unwrap();

            if node.borrow().val <= p_node.borrow().val {
                Solution::inorder_successor(node.borrow().right.clone(), Some(p_node))
            } else {
                let left = Solution::inorder_successor(node.borrow().left.clone(), Some(p_node));

                if left.is_some() {
                    left
                } else {
                    Some(node)
                }
            }
        }
    }
}

// Given a node, the inorder successor is most likely the leftmost node in the rightsubtree
// if the rightmost node doesn't exist, then we have a case to check
// if the rightmost node doesn't exist, then we have to make sure that the parent node
// exists and is greater than the current target node
// if that isnt the case, then we can return null
// I'm clearly weaker with recursive problems.
