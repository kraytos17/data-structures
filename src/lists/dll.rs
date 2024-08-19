use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    data: T,
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
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
        let node = Node::new(val);
        match self.head.take() {
            Some(h) => {
                h.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(h);
                self.head = Some(node);
            }
            None => {
                self.tail = Some(Rc::clone(&node));
                self.head = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let node = Node::new(val);
        match self.tail.take() {
            Some(t) => {
                t.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(t);
                self.tail = Some(node);
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|h| {
            if let Some(next) = h.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.len -= 1;

            Rc::try_unwrap(h).ok().unwrap().into_inner().data
        })
    }
}
