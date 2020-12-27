#![allow(dead_code)]

struct Solution {}

use crate::datastructures::linked_list::ListNode;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            None
        } else if l1.is_none() {
            l2
        } else if l2.is_none() {
            l1
        } else {
            let mut l1_node = l1.unwrap();
            let mut l2_node = l2.unwrap();

            if l1_node.val < l2_node.val {
                l1_node.next = Solution::merge_two_lists(l1_node.next, Some(l2_node));
                Some(l1_node)
            } else {
                l2_node.next = Solution::merge_two_lists(Some(l1_node), l2_node.next);
                Some(l2_node)
            }
        }
    }
}
