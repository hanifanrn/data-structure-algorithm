use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Rc<RefCell<Node>>;
type WeakLink = Weak<RefCell<Node>>;

#[derive(Clone, Debug)]
struct Node {
    value: i32,
    prev: Option<WeakLink>,
    next: Option<Link>,
}

impl Node {
    fn new(value: i32) -> Link {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Clone, Debug)]
struct DoublyLinkedList {
    head: Option<Link>,
    tail: Option<Link>,
    length: usize,
}

impl DoublyLinkedList {
    pub fn new(value: i32) -> Self {
        let node = Node::new(value);

        DoublyLinkedList {
            head: Some(node.clone()),
            tail: Some(node.clone()),
            length: 1,
        }
    }

    pub fn print_list(&self) {
        let mut temp = self.head.clone();
        while temp.is_some() {
            println!("{}", temp.as_ref().unwrap().borrow().value);
            temp = temp.unwrap().as_ref().borrow().next.clone();
        }
    }

    pub fn get_head(&self) {
        if !(self.head.is_some()) {
            println!("Head: None");
        } else {
            println!("Head: {}", self.head.as_ref().unwrap().borrow().value);
        }
    }

    pub fn get_tail(&self) {
        if !(self.tail.is_some()) {
            println!("Tail: None");
        } else {
            println!("Tail: {}", self.tail.as_ref().unwrap().borrow().value);
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
            new_node.borrow_mut().prev = Some(Rc::downgrade(&self.tail.clone().unwrap()));
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
            self.tail = self
                .tail
                .clone()
                .unwrap()
                .borrow()
                .prev
                .clone()
                .unwrap()
                .upgrade();
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
            self.head.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&new_node.clone()));
            self.head = Some(new_node.clone());
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
            self.head = self.head.clone().as_ref().unwrap().borrow().next.clone();
            self.head.as_mut().unwrap().borrow_mut().prev = None;
        }

        self.length -= 1;
    }

    pub fn get(&self, index: usize) -> Option<Link> {
        if index >= self.length {
            return None;
        } else if index < self.length / 2 {
            let mut temp = self.head.clone();
            for _i in 0..index {
                temp = temp.unwrap().as_ref().borrow().next.clone();
            }

            return temp;
        } else {
            let mut temp = self.tail.clone();
            let mut dll_idx = self.length - 1;
            while dll_idx > index {
                temp = temp
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .prev
                    .clone()
                    .unwrap()
                    .upgrade();
                dll_idx -= 1;
            }

            return temp;
        }
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let temp = self.get(index);

        match temp {
            Some(node) => {
                node.borrow_mut().value = value;
                return true;
            }
            None => return false,
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
            let mut before = self.get(index - 1);
            let mut after = before.as_ref().unwrap().borrow().next.clone();
            before.as_mut().unwrap().borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(Rc::downgrade(&before.clone().unwrap()));
            new_node.borrow_mut().next = after.clone();
            after.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&new_node.clone()));
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
            let mut temp = self.get(index);

            temp.clone()
                .unwrap()
                .borrow_mut()
                .next
                .as_mut()
                .unwrap()
                .borrow_mut()
                .prev = temp.as_ref().unwrap().borrow().prev.clone();

            temp.clone()
                .unwrap()
                .borrow_mut()
                .prev
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow_mut()
                .next = temp.as_ref().unwrap().borrow().next.clone();

            self.length -= 1;
        }
    }
}

fn main() {
    let mut my_dll = DoublyLinkedList::new(0);
    my_dll.print_list();
    my_dll.append(1);
    my_dll.append(2);
    my_dll.get_tail();
    println!("{:#?}", my_dll);
    my_dll.delete_last();
    println!("{:#?}", my_dll);
    my_dll.prepend(-1);
    println!("{:#?}", my_dll);
}

#[cfg(test)]
mod test {
    use crate::DoublyLinkedList;

    #[test]
    fn append_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.append(1);
        assert_eq!(0, dll.head.clone().unwrap().borrow().value);
        assert_eq!(1, dll.tail.clone().unwrap().borrow().value);
        dll.append(2);
        assert_eq!(2, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, dll.length);
    }

    #[test]
    fn delete_last_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.delete_last();
        assert_eq!(false, dll.head.is_some());
        assert_eq!(false, dll.tail.is_some());
        dll.delete_last();
        assert_eq!(false, dll.head.is_some());
        assert_eq!(false, dll.tail.is_some());
        dll.append(1);
        dll.append(2);
        dll.append(3);
        assert_eq!(3, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, dll.length);
        dll.delete_last();
        assert_eq!(2, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, dll.length);
    }

    #[test]
    fn prepend_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.prepend(-1);
        assert_eq!(-1, dll.head.clone().unwrap().borrow().value);
        assert_eq!(0, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, dll.length);
    }

    #[test]
    fn delete_first_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.delete_first();
        assert_eq!(false, dll.head.is_some());
        assert_eq!(false, dll.tail.is_some());
        dll.delete_first();
        assert_eq!(false, dll.head.is_some());
        assert_eq!(false, dll.tail.is_some());
        dll.append(1);
        dll.append(2);
        dll.append(3);
        assert_eq!(3, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, dll.length);
        dll.delete_first();
        assert_eq!(2, dll.head.clone().unwrap().borrow().value);
        assert_eq!(3, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, dll.length);
    }

    #[test]
    fn get_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.append(1);
        dll.append(2);
        assert_eq!(3, dll.length);
        let first_node = dll.get(0);
        let second_node = dll.get(1);
        let last_node = dll.get(2);
        let out_of_index_node = dll.get(3);
        assert_eq!(0, first_node.unwrap().borrow().value);
        assert_eq!(1, second_node.unwrap().borrow().value);
        assert_eq!(2, last_node.unwrap().borrow().value);
        assert_eq!(false, out_of_index_node.is_some());
    }

    #[test]
    fn set_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.append(1);
        dll.append(2);
        assert_eq!(3, dll.length);
        let set_first_node = dll.set(0, 9);
        let set_out_of_index_node = dll.set(3, 9);
        assert_eq!(true, set_first_node);
        assert_eq!(false, set_out_of_index_node);
    }

    #[test]
    fn insert_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.delete_last();
        assert_eq!(0, dll.length);
        let insert_zero = dll.insert(0, 0);
        assert_eq!(true, insert_zero);
        assert_eq!(1, dll.length);
        assert_eq!(0, dll.head.clone().unwrap().borrow().value);
        assert_eq!(0, dll.tail.clone().unwrap().borrow().value);
        let insert_first_node = dll.insert(0, -1);
        let insert_last_node = dll.insert(2, 1);
        let insert_out_of_index_node = dll.insert(4, 1);
        assert_eq!(true, insert_first_node);
        assert_eq!(true, insert_last_node);
        assert_eq!(false, insert_out_of_index_node);
    }

    #[test]
    fn delete_index_test() {
        let mut dll = DoublyLinkedList::new(0);
        dll.delete_node(0);
        assert_eq!(false, dll.head.is_some());
        assert_eq!(false, dll.tail.is_some());
        dll.delete_node(0);
        assert_eq!(false, dll.head.is_some());
        assert_eq!(false, dll.tail.is_some());
        dll.append(1);
        dll.append(2);
        dll.append(3);
        assert_eq!(3, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, dll.length);
        dll.delete_node(3);
        assert_eq!(1, dll.head.clone().unwrap().borrow().value);
        assert_eq!(3, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(3, dll.length);
        dll.delete_node(2);
        assert_eq!(1, dll.head.clone().unwrap().borrow().value);
        assert_eq!(2, dll.tail.clone().unwrap().borrow().value);
        assert_eq!(2, dll.length);
    }
}
