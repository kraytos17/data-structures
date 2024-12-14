use std::{cell::RefCell, rc::Rc};

pub type Link<T> = Option<Rc<RefCell<AVLNode<T>>>>;

#[derive(Debug)]
pub(crate) struct AVLNode<T> {
    left: Link<T>,
    right: Link<T>,
    data: T,
    height: i32,
}

#[derive(Debug)]
pub struct Avl<T> {
    root: Link<T>,
}

impl<T> AVLNode<T> {
    fn new(data: T) -> Rc<RefCell<AVLNode<T>>> {
        Rc::new(RefCell::new(AVLNode {
            left: None,
            right: None,
            data,
            height: 1,
        }))
    }
}

impl<T: PartialOrd + Clone> Avl<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) {
        let node = AVLNode::new(val);
        match self.root {
            Some(ref root) => Self::insert_node(root, node),
            None => self.root = Some(node),
        }
    }

    fn insert_node(curr: &Rc<RefCell<AVLNode<T>>>, node: Rc<RefCell<AVLNode<T>>>) {
        
    }
}
