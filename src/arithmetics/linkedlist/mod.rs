use std::cell::Cell;

use self::list_node::{Link, ListNode};

pub mod list_node;
pub mod opt;

pub struct LinkList<T> {
    head: Link<T>,
    size: Cell<usize>,
}

pub struct Iter<'a, T> {
    next: Option<&'a ListNode<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut ListNode<T>>,
}

impl<'a, T> LinkList<T> {
    pub fn new() -> Self {
        LinkList {
            head: None,
            size: Cell::new(0),
        }
    }

    pub fn push(&mut self, val: T) {
        self.size.set(self.size.get() + 1);
        self.head = Some(Box::new(ListNode {
            val: val,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.size.set(self.size.get() - 1);
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size.get()
    }
}

impl<T> From<Vec<T>> for LinkList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut link: LinkList<T> = LinkList::new();
        for item in value {
            link.push(item);
        }
        link
    }
}

impl<T> Iterator for LinkList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

impl<T> Drop for LinkList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use crate::arithmetics::linkedlist::list_node::ListNode;

    use super::LinkList;

    #[test]
    fn test_new() {
        let link: LinkList<i32> = LinkList::new();
        assert_eq!(link.head, None);
    }

    #[test]
    fn test_link_from() {
        let link = LinkList::from(vec![1, 2]);
        assert_eq!(
            link.head,
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        )
    }

    #[test]
    fn test_push() {
        let mut link = LinkList::new();
        link.push(1);
        link.push(2);
        link.push(3);

        let link2 = LinkList::from(vec![1, 2, 3]);
        assert_eq!(link.head, link2.head);
    }

    #[test]
    fn test_pop() {
        let mut link = LinkList::new();
        link.push(1);
        link.push(2);
        link.push(3);
        link.pop();
        let link2 = LinkList::from(vec![1, 2]);
        assert_eq!(link.head, link2.head);
    }

    #[test]
    fn test_len() {
        let mut link = LinkList::from(vec![1, 2, 3]);
        assert_eq!(link.len(), 3);

        link.pop();
        assert_eq!(link.len(), 2);

        link.pop();
        assert_eq!(link.len(), 1);

        link.pop();
        assert_eq!(link.len(), 0);

        link.pop();
        assert_eq!(link.len(), 0);

        link.push(1);
        assert_eq!(link.len(), 1);
    }

    #[test]
    fn test_peek() {
        let mut link = LinkList::from(vec![1, 2, 3]);
        assert_eq!(link.peek(), Some(&3));

        link.pop();
        assert_eq!(link.peek(), Some(&2));

        link.pop();
        assert_eq!(link.peek(), Some(&1));

        link.pop();
        assert_eq!(link.peek(), None);
    }

    #[test]
    fn test_peek_mut() {
        let mut link = LinkList::from(vec![1, 2, 3]);
        {
            let top = link.peek_mut();
            if let Some(top_val) = top {
                *top_val = *top_val + 1;
            }
            assert_eq!(link.peek(), Some(&4));
        }
        {
            let top2 = link.peek_mut();
            if let Some(top_val) = top2 {
                *top_val = *top_val * 2;
            }
            assert_eq!(link.peek(), Some(&8));
        }
    }

    #[test]
    fn test_link_iterator() {
        let link = LinkList::from(vec![1, 2, 3]);
        let mut val = 3;
        for item in link {
            assert_eq!(val, item);
            val -= 1;
        }
    }

    #[test]
    fn test_link_iter() {
        let link = LinkList::from(vec![1, 2, 3]);

        let mut val = 3;
        for item in link.iter() {
            assert_eq!(&val, item);
            val -= 1;
        }
    }

    #[test]
    fn test_link_iter_mut() {
        let mut link = LinkList::from(vec![1, 2, 3]);
        for item in link.iter_mut() {
            *item = *item * 2;
        }

        let mut val = 3;
        for item in link.iter() {
            assert_eq!(&(val * 2), item);
            val -= 1;
        }
    }
}
