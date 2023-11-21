use std::borrow::BorrowMut;

use super::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode<i32>>>) -> Vec<i32> {
    if let Some(mut head) = head {
        let mut list: Vec<i32> = Vec::new();
        list.push(head.val);
        while let Some(next) = head.next.take() {
            list.insert(0, next.val);
            head = next;
        }
        return list;
    }
    vec![]
}

pub fn reverse_list_by_recursive(head: Option<Box<ListNode<i32>>>) -> Vec<i32> {
    if let Some(head) = head {
        let mut list: Vec<i32> = vec![];
        fn recursive(node: Box<ListNode<i32>>, list: &mut Vec<i32>) {
            if let Some(next) = node.next {
                recursive(next, list);
            }
            list.push(node.val);
        }
        recursive(head, &mut list);
        return list;
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::arithmetics::linkedlist::{
        common::{reverse_list, reverse_list_by_recursive},
        list_node::ListNode,
    };

    #[test]
    fn test_reverse_list_by_recursive() {
        let linkedlist = ListNode::from_list(vec![1, 2, 3]);

        assert_eq!(reverse_list_by_recursive(linkedlist), vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_list() {
        let linkedlist = ListNode::from_list(vec![1, 2, 3]);

        assert_eq!(reverse_list(linkedlist), vec![3, 2, 1]);
    }
}
