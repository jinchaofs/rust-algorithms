use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T>
where
    T: Clone + PartialEq + Debug,
{
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_list(list: Vec<T>) -> Option<Box<ListNode<T>>> {
        let mut head: Option<Box<ListNode<T>>> = None;
        let mut current = &mut head;

        for val in list {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod test {
    use crate::arithmetics::linkedlist::list_node::ListNode;

    #[test]
    fn from_list() {
        let mut linked = Box::new(ListNode::new(1));
        linked.next = Some(Box::new(ListNode::new(2)));
        assert_eq!(ListNode::from_list(vec![1, 2]), Some(linked));
    }
    #[test]
    fn test_reverse_list() {}
}
