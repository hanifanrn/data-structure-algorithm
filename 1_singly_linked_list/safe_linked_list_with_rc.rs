// safe linkedlist with ref count
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

    pub fn get(&self, index: usize) -> Option<Link> {
        match self.length as isize - index as isize {
            0.. => {
                let mut temp = self.head.clone();

                for _i in 0..index {
                    temp = temp.clone().unwrap().borrow().next.clone();
                }
                return temp;
            }
            _ => return None,
        }
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let temp = self.get(index);

        match temp {
            Some(node) => {
                node.borrow_mut().value = value;
                return true;
            }
            None => {
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
            let temp = self.get(index - 1);

            new_node.borrow_mut().next = temp.as_ref().unwrap().borrow().next.clone();
            temp.unwrap().borrow_mut().next = Some(new_node);

            self.length += 1;
            return true;
        }
    }

    pub fn delete_node(&mut self, index: usize) {
        if index >= self.length {
            return;
        } else if index == 0 {
            self.delete_first();
        } else if index == self.length - 1 {
            self.delete_last();
        } else {
            let mut prev = self.get(index - 1);
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
    use crate::LinkedList;

    #[test]
    fn append_test() {
        let mut ll = LinkedList::new(0);
        ll.append(1);
        assert_eq!(0, ll.head.clone().unwrap().borrow().value);
        assert_eq!(1, ll.tail.clone().unwrap().borrow().value);
        ll.append(2);
        assert_eq!(2, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, ll.length);
    }

    #[test]
    fn delete_last_test() {
        let mut ll = LinkedList::new(0);
        ll.delete_last();
        assert_eq!(false, ll.head.is_some());
        assert_eq!(false, ll.tail.is_some());
        ll.delete_last();
        assert_eq!(false, ll.head.is_some());
        assert_eq!(false, ll.tail.is_some());
        ll.append(1);
        ll.append(2);
        ll.append(3);
        assert_eq!(3, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, ll.length);
        ll.delete_last();
        assert_eq!(2, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, ll.length);
    }

    #[test]
    fn prepend_test() {
        let mut ll = LinkedList::new(0);
        ll.prepend(-1);
        assert_eq!(-1, ll.head.clone().unwrap().borrow().value);
        assert_eq!(0, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, ll.length);
    }

    #[test]
    fn delete_first_test() {
        let mut ll = LinkedList::new(0);
        ll.delete_first();
        assert_eq!(false, ll.head.is_some());
        assert_eq!(false, ll.tail.is_some());
        ll.delete_first();
        assert_eq!(false, ll.head.is_some());
        assert_eq!(false, ll.tail.is_some());
        ll.append(1);
        ll.append(2);
        ll.append(3);
        assert_eq!(3, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, ll.length);
        ll.delete_first();
        assert_eq!(2, ll.head.clone().unwrap().borrow().value);
        assert_eq!(3, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, ll.length);
    }

    #[test]
    fn get_test() {
        let mut ll = LinkedList::new(0);
        ll.append(1);
        ll.append(2);
        assert_eq!(3, ll.length);
        let first_node = ll.get(0);
        let second_node = ll.get(1);
        let last_node = ll.get(2);
        let out_of_index_node = ll.get(3);
        assert_eq!(0, first_node.unwrap().borrow().value);
        assert_eq!(1, second_node.unwrap().borrow().value);
        assert_eq!(2, last_node.unwrap().borrow().value);
        assert_eq!(false, out_of_index_node.is_some());
    }

    #[test]
    fn set_test() {
        let mut ll = LinkedList::new(0);
        ll.append(1);
        ll.append(2);
        assert_eq!(3, ll.length);
        let set_first_node = ll.set(0, 9);
        let set_out_of_index_node = ll.set(3, 9);
        assert_eq!(true, set_first_node);
        assert_eq!(false, set_out_of_index_node);
    }

    #[test]
    fn insert_test() {
        let mut ll = LinkedList::new(0);
        ll.delete_last();
        assert_eq!(0, ll.length);
        let insert_zero = ll.insert(0, 0);
        assert_eq!(true, insert_zero);
        assert_eq!(1, ll.length);
        assert_eq!(0, ll.head.clone().unwrap().borrow().value);
        assert_eq!(0, ll.tail.clone().unwrap().borrow().value);
        let insert_first_node = ll.insert(0, -1);
        let insert_last_node = ll.insert(2, 1);
        let insert_out_of_index_node = ll.insert(4, 1);
        assert_eq!(true, insert_first_node);
        assert_eq!(true, insert_last_node);
        assert_eq!(false, insert_out_of_index_node);
    }

    #[test]
    fn delete_index_test() {
        let mut ll = LinkedList::new(0);
        ll.delete_node(0);
        assert_eq!(false, ll.head.is_some());
        assert_eq!(false, ll.tail.is_some());
        ll.delete_node(0);
        assert_eq!(false, ll.head.is_some());
        assert_eq!(false, ll.tail.is_some());
        ll.append(1);
        ll.append(2);
        ll.append(3);
        assert_eq!(3, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, ll.length);
        ll.delete_node(3);
        assert_eq!(1, ll.head.clone().unwrap().borrow().value);
        assert_eq!(3, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, ll.length);
        ll.delete_node(2);
        assert_eq!(1, ll.head.clone().unwrap().borrow().value);
        assert_eq!(2, ll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, ll.length);
    }
    #[test]
    fn reverse_test() {
        let mut ll = LinkedList::new(0);
        ll.append(1);
        ll.append(2);
        ll.append(3);
        ll.reverse();
        let first_node = ll.get(0);
        let second_node = ll.get(1);
        let third_node = ll.get(2);
        let last_node = ll.get(3);
        assert_eq!(3, first_node.unwrap().borrow().value);
        assert_eq!(2, second_node.unwrap().borrow().value);
        assert_eq!(1, third_node.unwrap().borrow().value);
        assert_eq!(0, last_node.unwrap().borrow().value);
        assert_eq!(3, ll.head.clone().unwrap().borrow().value);
        assert_eq!(0, ll.tail.clone().unwrap().borrow().value);
    }
}
