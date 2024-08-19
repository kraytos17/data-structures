use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    next: Link<T>,
    prev: Option<Weak<RefCell<Node<T>>>>,
    data: T,
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Option<Weak<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            data,
        }))
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = Node::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_head));
                self.head = Some(new_head);
            }
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = Node::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                if let Some(old_tail_strong) = old_tail.upgrade() {
                    old_tail_strong.borrow_mut().next = Some(Rc::clone(&new_tail));
                    new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail_strong));
                    self.tail = Some(Rc::downgrade(&new_tail));
                }
            }
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(Rc::downgrade(&new_tail));
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().and_then(|old_tail| {
            old_tail.upgrade().map(|old_tail_strong| {
                match old_tail_strong.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        if let Some(new_tail_strong) = new_tail.upgrade() {
                            new_tail_strong.borrow_mut().next = None;
                            self.tail = Some(Rc::downgrade(&new_tail_strong));
                        }
                    }
                    None => {
                        self.head = None;
                    }
                }
                self.len -= 1;
                Rc::try_unwrap(old_tail_strong)
                    .ok()
                    .unwrap()
                    .into_inner()
                    .data
            })
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| unsafe { &(*node.as_ptr()).data })
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail
            .as_ref()
            .and_then(|weak| weak.upgrade())
            .map(|node| unsafe { &(*node.as_ptr()).data })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Rc<RefCell<Node<T>>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            let node_ref = unsafe { &*node.as_ptr() };
            self.next = node_ref.next.as_ref();
            &node_ref.data
        })
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            next: self.head.as_ref(),
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.peek_front(), Some(&3));
        assert_eq!(list.peek_back(), Some(&1));
    }

    #[test]
    fn test_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));
    }

    #[test]
    fn test_pop_front() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek_front(), Some(&2));

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek_back(), Some(&2));

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);

        list.push_back(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));

        list.push_back(2);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&2));
    }

    #[test]
    fn test_clear() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let collected: Vec<&i32> = list.into_iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }
}
