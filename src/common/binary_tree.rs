use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<BinaryTree<T>>>>,
    pub right: Option<Rc<RefCell<BinaryTree<T>>>>,
}

impl<T> BinaryTree<T>
where
    T: PartialOrd,
{
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    fn insert_node(&mut self, node: BinaryTree<T>) {
        if self.value > node.value {
            if self.left.is_some() {
                self.left.as_mut().unwrap().borrow_mut().insert_node(node);
            } else {
                self.left = Some(Rc::new(RefCell::new(node)));
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().borrow_mut().insert_node(node);
            } else {
                self.right = Some(Rc::new(RefCell::new(node)));
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        let new_node = BinaryTree::new(value);
        self.insert_node(new_node);
    }

    // pub fn find(&self, search: &impl Fn(&T) -> bool) -> Option<&BinaryTree<T>> {
    //     if search(&self.value) {
    //         return Some(self);
    //     }
    //     if let Some(ref left) = self.left {
    //         if let Some(found) = left.borrow().find(search) {
    //             return Some(found);
    //         }
    //     }
    //     if let Some(ref right) = self.right {
    //         if let Some(found) = right.borrow().find(search) {
    //             return Some(found);
    //         }
    //     }
    //     None
    // }

    pub fn find(
        self_rc: &Rc<RefCell<BinaryTree<T>>>,
        search: &impl Fn(&T) -> bool,
    ) -> Vec<Rc<RefCell<BinaryTree<T>>>> {
        let mut res = Vec::new();

        if let Some(left) = &self_rc.borrow().left {
            res.extend(Self::find(left, search));
        }
        if let Some(right) = &self_rc.borrow().right {
            res.extend(Self::find(right, search));
        }

        if search(&self_rc.borrow().value) {
            res.push(self_rc.clone());
        }
        res
    }
}
