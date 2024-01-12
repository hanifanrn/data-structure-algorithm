// unsafe linked list in rust using raw pointer and unsafe rust

use std::ptr;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    value: i32,
    next: *mut Node,
}

impl Node {
    fn new(value: i32) -> *mut Self {
        Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LinkedList {
    head: *mut Node,
    tail: *mut Node,
    length: i32,
}

impl LinkedList {
    pub fn new(value: i32) -> Self {
        let new_node = Node::new(value);

        LinkedList {
            head: new_node,
            tail: new_node,
            length: 1,
        }
    }

    pub fn print_list(&self) {
        let mut temp = self.head;
        while !(temp.is_null()) {
            unsafe {
                println!("{}", (*temp).value);
                temp = (*temp).next;
            }
        }
    }

    pub fn get_head(&self) {
        if self.head.is_null() {
            println!("Head: null pointer");
        } else {
            unsafe {
                println!("Head: {}", (*self.head).value);
            }
        }
    }

    pub fn get_tail(&self) {
        if self.head.is_null() {
            println!("Tail: null pointer");
        } else {
            unsafe {
                println!("Tail: {}", (*self.tail).value);
            }
        }
    }

    pub fn get_length(&self) {
        println!("Length: {}", self.length);
    }

    pub fn append(&mut self, value: i32) {
        let new_node = Node::new(value);

        if self.length == 0 {
            self.head = new_node;
            self.tail = new_node;
        } else {
            unsafe {
                (*self.tail).next = new_node;
            }
            self.tail = new_node;
        }

        self.length += 1;
    }

    pub fn delete_last(&mut self) {
        let mut temp = self.head;

        if self.length == 0 {
        } else if self.length == 1 {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            let mut pre = self.head;

            unsafe {
                while !((*temp).next.is_null()) {
                    pre = temp;
                    temp = (*temp).next;
                }
            }

            self.tail = pre;

            unsafe {
                (*self.tail).next = ptr::null_mut();
            }
        }
        unsafe {
            drop(Box::from_raw(temp));
        }

        self.length -= 1;
    }

    pub fn prepend(&mut self, value: i32) {
        let new_node = Node::new(value);

        if self.length == 0 {
            self.head = new_node;
            self.tail = new_node;
        } else {
            unsafe {
                (*new_node).next = self.head;
            }
            self.head = new_node;
        }

        self.length += 1;
    }
}

fn main() {
    let mut my_linked_list = LinkedList::new(1);

    my_linked_list.append(2);
    my_linked_list.print_list();
    my_linked_list.get_head();
    my_linked_list.get_tail();
    my_linked_list.get_length();

    my_linked_list.append(3);

    my_linked_list.print_list();
    my_linked_list.get_length();

    println!("delete list");
    my_linked_list.delete_last();
    my_linked_list.print_list();
    my_linked_list.get_tail();

    println!("prepend list");
    my_linked_list.prepend(9);
    my_linked_list.get_head();
    my_linked_list.print_list();
}
