use std::cmp::Ordering;

use super::list_node::ListNode;

pub fn reverse(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    let mut head = head;
    let mut prev = None;
    while let Some(mut temp) = head.take() {
        head = temp.next.take();
        temp.next = prev;
        prev = Some(temp);
    }
    prev
}

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

/// The function `delete_node` takes a linked list and a value, and removes all nodes with that value
/// from the list.
///
/// Arguments:
///
/// * `head`: The `head` parameter is an `Option` type that represents the head of a linked list. It is
/// a box that contains a `ListNode` struct with a generic type `i32`.
/// * `val`: The `val` parameter represents the value of the node that needs to be deleted from the
/// linked list.
///
/// Returns:
///
/// The function `delete_node` returns an `Option<Box<ListNode<i32>>>`.
pub fn delete_node(head: Option<Box<ListNode<i32>>>, val: i32) -> Option<Box<ListNode<i32>>> {
    if head.is_none() {
        return None;
    }
    let mut head = head;
    let mut current = &mut head;
    loop {
        match current {
            None => break,
            Some(node) if node.val == val => {
                *current = node.next.take();
            }
            Some(node) => {
                current = &mut node.next;
            }
        }
    }
    head
}

/// The function `find_nth_from_end` takes a linked list and an index `cnt`, and returns the nth node
/// from the end of the list.
///
/// Arguments:
///
/// * `head`: The `head` parameter is an `Option` containing a `Box` that holds a `ListNode` of type
/// `i32`. It represents the head of a linked list.
/// * `cnt`: The `cnt` parameter represents the position from the end of the linked list that we want to
/// find. For example, if `cnt` is 0, it means we want to find the last element of the linked list. If
/// `cnt` is 1, it means we want to find the last node of the linked list.
///
/// Returns:
///
/// The function `find_nth_from_end` returns an `Option<Box<ListNode<i32>>>`.
pub fn find_nth_from_end(
    head: Option<Box<ListNode<i32>>>,
    cnt: usize,
) -> Option<Box<ListNode<i32>>> {
    let mut head = head;
    let mut length = 0;
    let mut current = &mut head;
    while let Some(node) = current {
        length += 1;
        current = &mut node.next;
    }
    current = &mut head;
    let mut count = 0;
    while let Some(node) = current {
        if cnt + count == length {
            return Some(Box::new(ListNode {
                val: node.val,
                next: node.next.take(),
            }));
        }
        count += 1;
        current = &mut node.next;
    }
    None
}

/// The function `find_nth_from_end_v2` takes a linked list and an index `cnt`, and returns the nth node
/// from the end of the list.
///
/// Arguments:
///
/// * `head`: The `head` parameter is an `Option` that represents the head of a linked list. It is of
/// type `Option<Box<ListNode<i32>>>`, which means it can either be `Some` containing a boxed `ListNode`
/// or `None` representing an empty list.
/// * `cnt`: The `cnt` parameter represents the position of the node from the end of the linked list
/// that we want to find. For example, if `cnt` is 1, it means we want to find the last node of the
/// linked list.
///
/// Returns:
///
/// The function `find_nth_from_end_v2` returns an `Option<Box<ListNode<i32>>>`.
pub fn find_nth_from_end_v2(
    head: Option<Box<ListNode<i32>>>,
    cnt: usize,
) -> Option<Box<ListNode<i32>>> {
    let mut fast = &head;
    let mut slow = &head;

    for _ in 0..cnt {
        if let Some(node) = fast {
            fast = &node.next;
        }
    }

    while let Some(node) = fast {
        fast = &node.next;
        slow = &slow.as_ref()?.next;
    }
    slow.clone()
}

pub fn merge(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    let (mut l1, mut l2) = (&mut l1, &mut l2);

    while let (Some(node1), Some(node2)) = (&mut l1, &mut l2) {
        match node1.val.cmp(&node2.val) {
            Ordering::Less => {
                current.next = Some(Box::new(ListNode::new(node1.val)));
                current = current.next.as_mut().unwrap();
                l1 = &mut l1.as_mut()?.next;
            }
            Ordering::Greater => {
                current.next = Some(Box::new(ListNode::new(node2.val)));
                current = current.next.as_mut().unwrap();
                l2 = &mut l2.as_mut()?.next;
            }
            Ordering::Equal => {
                current.next = Some(Box::new(ListNode::new(node1.val)));
                current = current.next.as_mut().unwrap();
                current.next = Some(Box::new(ListNode::new(node2.val)));
                current = current.next.as_mut().unwrap();
                l1 = &mut l1.as_mut()?.next;
                l2 = &mut l2.as_mut()?.next;
            }
        }
    }

    if let Some(node) = l1.take() {
        current.next = Some(node);
    }
    if let Some(node) = l2.take() {
        current.next = Some(node);
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_delete_node() {
        let linkedlist = ListNode::from_list(vec![1, 2, 3]);
        assert_eq!(delete_node(linkedlist, 2), ListNode::from_list(vec![1, 3]));
    }

    #[test]
    fn test_reverse() {
        let linkedlist = ListNode::from_list(vec![1, 2, 3]);

        assert_eq!(reverse(linkedlist), ListNode::from_list(vec![3, 2, 1]));
    }

    #[test]
    fn test_find_nth_from_end() {
        let linkedlist = ListNode::from_list(vec![1, 2, 3, 4]);
        assert_eq!(
            find_nth_from_end(linkedlist, 2),
            ListNode::from_list(vec![3, 4])
        );
    }

    #[test]
    fn test_find_nth_from_end_v2() {
        let linkedlist = ListNode::from_list(vec![1, 2, 3, 4]);
        assert_eq!(
            find_nth_from_end_v2(linkedlist, 2),
            ListNode::from_list(vec![3, 4])
        );
    }

    #[test]
    fn test_merge() {
        assert_eq!(
            merge(
                ListNode::from_list(vec![1, 2]),
                ListNode::from_list(vec![3])
            ),
            ListNode::from_list(vec![1, 2, 3])
        );
    }
}
