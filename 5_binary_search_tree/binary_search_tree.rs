use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<Node>>;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    value: i32,
    left: Option<Link>,
    right: Option<Link>,
}

impl Node {
    pub fn new(value: i32) -> Link {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BinarySearchTree {
    root: Option<Link>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: i32) -> bool {
        let new_node = Node::new(value);

        if self.root == None {
            self.root = Some(new_node);
            return true;
        }

        let mut temp = self.root.clone();
        loop {
            if temp.as_ref().unwrap().borrow().value == value {
                return false;
            }

            if temp.as_ref().unwrap().borrow().value < value {
                if temp.as_ref().unwrap().borrow().right == None {
                    temp.as_mut().unwrap().borrow_mut().right = Some(new_node);
                    return true;
                }
                temp = temp.clone().unwrap().borrow().right.clone();
            } else {
                if temp.as_ref().unwrap().borrow().left == None {
                    temp.as_mut().unwrap().borrow_mut().left = Some(new_node);
                    return true;
                }
                temp = temp.clone().unwrap().borrow().left.clone();
            }
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        if self.root == None {
            return false;
        }

        let mut temp = self.root.clone();
        while temp.is_some() {
            if temp.as_ref().unwrap().borrow().value < value {
                temp = temp.clone().unwrap().borrow().right.clone();
            } else if temp.as_ref().unwrap().borrow().value > value {
                temp = temp.clone().unwrap().borrow().left.clone();
            } else {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    let mut bst: BinarySearchTree = BinarySearchTree::new();
    bst.insert(1);
    bst.insert(-1);
    println!("{:#?}", bst);
    bst.insert(-1);
    bst.insert(3);
    println!("{:#?}", bst);
    let this_true1 = bst.contains(-1);
    let this_true2 = bst.contains(3);
    let this_false = bst.contains(9);
    println!("true: {this_true1}, true: {this_true2}, false: {this_false}")
}

#[cfg(test)]
mod test {
    use crate::BinarySearchTree;

    #[test]
    fn new_bst() {
        let my_bst = BinarySearchTree::new();
        let my_bst_value = my_bst.root;
        assert_eq!(None, my_bst_value);
    }

    #[test]
    fn insert_value() {
        let mut my_bst = BinarySearchTree::new();
        my_bst.insert(0);
        let my_bst_value = my_bst.root.as_ref().unwrap().borrow().value;
        assert_eq!(0, my_bst_value);
    }

    #[test]
    fn insert_value_child_node() {
        let mut my_bst = BinarySearchTree::new();
        my_bst.insert(0);
        my_bst.insert(-1);
        my_bst.insert(1);
        my_bst.insert(-2);
        my_bst.insert(2);
        let left_node_root = my_bst.root.as_ref().unwrap().borrow().left.clone().unwrap();
        let mut eq_left_node_root = BinarySearchTree::new();
        eq_left_node_root.insert(-1);
        eq_left_node_root.insert(-2);
        assert_eq!(*eq_left_node_root.root.as_ref().unwrap(), left_node_root);
    }

    #[test]
    fn contains_test() {
        let mut my_bst = BinarySearchTree::new();
        my_bst.insert(-1);
        my_bst.insert(100);
        my_bst.insert(50);
        my_bst.insert(75);
        let contain_true = my_bst.contains(75);
        let contain_false = my_bst.contains(-100);
        let contain_true_root = my_bst.contains(-1);
        assert_eq!(true, contain_true_root);
        assert_eq!(false, contain_false);
        assert_eq!(true, contain_true);
    }
}
