// safe linkedlist with ref count but a lot of clone

use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    value: i32,
    next: Option<Link>,
}

impl Node {
    fn new(value: i32) -> Link {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Clone, Debug)]
struct LinkedList {
    head: Option<Link>,
    tail: Option<Link>,
    length: usize,
}

impl LinkedList {
    pub fn new(value: i32) -> Self {
        let new_node = Node::new(value);

        LinkedList {
            head: Some(new_node.clone()),
            tail: Some(new_node.clone()),
            length: 1,
        }
    }

    pub fn print_list(&self) {
        let mut temp = self.head.clone();
        while temp.is_some() {
            println!("{}", temp.clone().unwrap().borrow().value);
            temp = temp.clone().unwrap().borrow().next.clone();
        }
    }

    pub fn get_head(&self) {
        if self.head.is_some() {
            println!("Head: {}", self.head.as_ref().unwrap().borrow().value);
        } else {
            println!("Head: None");
        }
    }

    pub fn get_tail(&self) {
        if self.tail.is_some() {
            println!("Tail: {}", self.tail.as_ref().unwrap().borrow().value);
        } else {
            println!("Tail: None");
        }
    }

    pub fn get_length(&self) {
        println!("Length: {}", self.length);
    }

    pub fn append(&mut self, value: i32) {
        let new_node = Node::new(value);

        if self.length == 0 {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            self.tail.as_mut().unwrap().borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
        }

        self.length += 1;
    }

    pub fn delete_last(&mut self) {
        if self.length == 0 {
            return;
        } else if self.length == 1 {
            self.head = None;
            self.tail = None;
        } else {
            let mut prev = self.head.clone();
            let mut temp = self.head.clone();

            while temp.as_ref().unwrap().borrow().next.is_some() {
                prev = temp.clone();
                temp = temp.clone().unwrap().borrow().next.clone();
            }

            self.tail = prev.clone();
            self.tail.as_mut().unwrap().borrow_mut().next = None;
        }

        self.length -= 1;
    }

    pub fn prepend(&mut self, value: i32) {
        let new_node = Node::new(value);

        if self.length == 0 {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            new_node.borrow_mut().next = self.head.clone();
            self.head = Some(new_node);
        }

        self.length += 1;
    }

    pub fn delete_first(&mut self) {
        if self.length == 0 {
            return;
        } else if self.length == 1 {
            self.head = None;
            self.tail = None;
        } else {
            self.head = self.head.clone().unwrap().borrow_mut().next.clone();
        }

        self.length -= 1;
    }

    pub fn get(&self, index: usize) -> Result<Option<Link>, &'static str> {
        match self.length as isize - index as isize {
            0.. => {
                let mut temp = self.head.clone();

                for _i in 0..index {
                    temp = temp.clone().unwrap().borrow().next.clone();
                }
                return Ok(temp);
            }
            _ => return Err("index out of bound"),
        }
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let temp = self.get(index);

        match temp {
            Ok(mut node) => {
                node.as_mut().unwrap().borrow_mut().value = value;
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }

    pub fn insert(&mut self, index: usize, value: i32) -> bool {
        if index > self.length {
            return false;
        } else if index == 0 {
            self.prepend(value);
            return true;
        } else if index == self.length {
            self.append(value);
            return true;
        } else {
            let new_node = Node::new(value);
            let mut temp = self.get(index - 1).unwrap();

            new_node.borrow_mut().next = temp.as_ref().unwrap().borrow().next.clone();
            temp.as_mut().unwrap().borrow_mut().next = Some(new_node);

            self.length += 1;
            return true;
        }
    }

    pub fn delete_node(&mut self, index: usize) {
        if index > self.length {
            return;
        } else if index == 0 {
            self.delete_first();
        } else if index == self.length {
            self.delete_last();
        } else {
            let mut prev = self.get(index - 1).unwrap();
            let temp = prev.as_ref().unwrap().borrow().next.clone();
            prev.as_mut().unwrap().borrow_mut().next = temp;

            self.length -= 1;
        }
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut temp = self.head.clone();
        self.head = self.tail.clone();
        self.tail = temp.clone();

        for _i in 0..self.length {
            let after = temp.as_ref().unwrap().borrow().next.clone();
            temp.as_mut().unwrap().borrow_mut().next = prev;
            prev = temp;
            temp = after;
        }
    }
}

fn main() {
    let mut my_linked_list = LinkedList::new(1);
    my_linked_list.print_list();
    my_linked_list.get_head();

    my_linked_list.append(2);
    my_linked_list.get_length();
    my_linked_list.get_tail();

    my_linked_list.append(3);
    println!("{:#?}", my_linked_list);
    my_linked_list.delete_last();
    my_linked_list.prepend(0);
    println!("{:#?}", my_linked_list);
    let first_node = my_linked_list.get(1);
    match first_node {
        Ok(v) => println!("{:#?}", v.unwrap()),
        Err(e) => println!("{:?}", e),
    }

    println!("change index 0 to 9");
    let set_0 = my_linked_list.set(0, 9);
    println!("{}", set_0);
    println!("{:#?}", my_linked_list);
    my_linked_list.insert(1, 8);
    my_linked_list.insert(2, 7);
    println!("delte first");
    my_linked_list.delete_first();
    let false_insert = my_linked_list.insert(99, 99);
    println!("{:#?}", my_linked_list);
    println!("{}", false_insert);

    my_linked_list.reverse();
    println!("{:#?}", my_linked_list);
}

#[cfg(test)]
mod test {
    use crate::{LinkedList, Node};

    #[test]
    fn delete_node_test() {
        let mut linked_list = LinkedList::new(0);
        linked_list.append(1);
        linked_list.append(2);
        linked_list.append(3);

        // delete index 2
        linked_list.delete_node(2);
        // check length
        assert_eq!(linked_list.length, 3);
    }
}
