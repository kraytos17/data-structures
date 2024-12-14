use std::{cell::RefCell, rc::Rc};

pub type Link<T> = Option<Rc<RefCell<BSTNode<T>>>>;

#[derive(Debug)]
pub(crate) struct BSTNode<T> {
    left: Link<T>,
    right: Link<T>,
    data: T,
}

#[derive(Debug)]
pub struct Bst<T> {
    root: Link<T>,
}

impl<T> BSTNode<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BSTNode {
            left: None,
            right: None,
            data,
        }))
    }
}

impl<T: PartialOrd + Clone> Bst<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) {
        let node = BSTNode::new(val);
        match self.root {
            Some(ref root) => Self::insert_node(root, node),
            None => self.root = Some(node),
        }
    }

    fn insert_node(curr: &Rc<RefCell<BSTNode<T>>>, node: Rc<RefCell<BSTNode<T>>>) {
        let mut borrowed_val = curr.borrow_mut();
        if node.borrow().data < borrowed_val.data {
            match borrowed_val.left {
                Some(ref left) => Self::insert_node(left, node),
                None => borrowed_val.left = Some(node),
            }
        } else {
            match borrowed_val.right {
                Some(ref right) => Self::insert_node(right, node),
                None => borrowed_val.right = Some(node),
            }
        }
    }

    pub fn search(&self, val: T) -> bool {
        Self::search_node(&self.root, val)
    }

    fn search_node(curr: &Link<T>, val: T) -> bool {
        match curr {
            Some(node) => {
                if val < node.borrow().data {
                    Self::search_node(&node.borrow().left, val)
                } else if val > node.borrow().data {
                    Self::search_node(&node.borrow().right, val)
                } else {
                    true
                }
            }
            None => false,
        }
    }

    pub fn inorder_traversal(&self, visit_fn: &mut impl FnMut(&T)) {
        Self::inorder(&self.root, visit_fn);
    }

    fn inorder(node: &Link<T>, visit_fn: &mut impl FnMut(&T)) {
        if let Some(ref n) = node {
            Self::inorder(&n.borrow().left, visit_fn);
            visit_fn(&n.borrow().data);
            Self::inorder(&n.borrow().right, visit_fn);
        }
    }

    pub fn preorder_traversal(&self, visit_fn: &mut impl FnMut(&T)) {
        Self::preorder(&self.root, visit_fn);
    }

    fn preorder(node: &Link<T>, visit_fn: &mut impl FnMut(&T)) {
        if let Some(ref n) = node {
            visit_fn(&n.borrow().data);
            Self::preorder(&n.borrow().left, visit_fn);
            Self::preorder(&n.borrow().right, visit_fn);
        }
    }

    pub fn postorder_traversal(&self, visit_fn: &mut impl FnMut(&T)) {
        Self::postorder(&self.root, visit_fn);
    }

    fn postorder(node: &Link<T>, visit_fn: &mut impl FnMut(&T)) {
        if let Some(ref n) = node {
            Self::postorder(&n.borrow().left, visit_fn);
            Self::postorder(&n.borrow().right, visit_fn);
            visit_fn(&n.borrow().data);
        }
    }

    pub fn delete(&mut self, val: T) {
        self.root = Self::delete_node(self.root.take(), val);
    }

    fn delete_node(node: Link<T>, val: T) -> Link<T> {
        match node {
            Some(ref n) => {
                if val < n.borrow().data {
                    let left = Rc::clone(n);
                    let new_left = Self::delete_node(n.borrow().left.clone(), val);
                    left.borrow_mut().left = new_left;
                    Some(left)
                } else if val > n.borrow().data {
                    let right = Rc::clone(n);
                    let new_right = Self::delete_node(n.borrow().right.clone(), val);
                    right.borrow_mut().right = new_right;
                    Some(right)
                } else {
                    if n.borrow().left.is_none() {
                        return n.borrow().right.clone();
                    } else if n.borrow().right.is_none() {
                        return n.borrow().left.clone();
                    }

                    let min = Self::find_min(n.borrow().right.as_ref().unwrap());
                    let new_data = min.borrow().data.clone();
                    let right = Rc::clone(n);
                    let new_right = Self::delete_node(n.borrow().right.clone(), new_data.clone());
                    right.borrow_mut().right = new_right;
                    right.borrow_mut().data = new_data;
                    Some(right)
                }
            }
            None => None,
        }
    }

    fn find_min(node: &Rc<RefCell<BSTNode<T>>>) -> Rc<RefCell<BSTNode<T>>> {
        match node.borrow().left {
            Some(ref left) => Self::find_min(left),
            None => Rc::clone(node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        assert!(tree.search(5));
        assert!(tree.search(3));
        assert!(tree.search(7));
        assert!(tree.search(2));
        assert!(tree.search(4));
        assert!(tree.search(6));
        assert!(tree.search(8));

        assert!(!tree.search(10));
        assert!(!tree.search(0));
    }

    #[test]
    fn test_inorder_traversal() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut elements = vec![];
        tree.inorder_traversal(&mut |x| elements.push(*x));

        assert_eq!(elements, vec![2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_preorder_traversal() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut elements = vec![];
        tree.preorder_traversal(&mut |x| elements.push(*x));

        assert_eq!(elements, vec![5, 3, 2, 4, 7, 6, 8]);
    }

    #[test]
    fn test_postorder_traversal() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut elements = vec![];
        tree.postorder_traversal(&mut |x| elements.push(*x));

        assert_eq!(elements, vec![2, 4, 3, 6, 8, 7, 5]);
    }

    #[test]
    fn test_delete_leaf_node() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);

        tree.delete(2);

        assert!(!tree.search(2));
        assert!(tree.search(3));
        assert!(tree.search(5));
        assert!(tree.search(7));
    }

    #[test]
    fn test_delete_node_with_one_child() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(6);

        tree.delete(7);

        assert!(!tree.search(7));
        assert!(tree.search(6));
        assert!(tree.search(3));
        assert!(tree.search(5));
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(6);
        tree.insert(8);

        tree.delete(7);

        assert!(!tree.search(7));
        assert!(tree.search(6));
        assert!(tree.search(8));
        assert!(tree.search(3));
        assert!(tree.search(5));
    }

    #[test]
    fn test_delete_root_node() {
        let mut tree = Bst::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        tree.delete(5);

        assert!(!tree.search(5));
        assert!(tree.search(3));
        assert!(tree.search(7));
    }
}
