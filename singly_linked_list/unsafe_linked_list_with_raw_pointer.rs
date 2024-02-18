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
    length: usize,
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
        if self.length == 0 {
            return;
        } else if self.length == 1 {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            let mut temp = self.head;
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

    pub fn delete_first(&mut self) {
        if self.length == 0 {
        } else if self.length == 1 {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            unsafe {
                self.head = (*self.head).next;
            }
        }

        self.length -= 1;
    }

    pub fn get(&self, index: usize) -> *mut Node {
        if index >= self.length {
            return ptr::null_mut();
        } else {
            let mut temp = self.head;

            for _i in 0..index {
                unsafe {
                    temp = (*temp).next;
                }
            }

            return temp;
        }
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let temp = self.get(index);

        if !(temp.is_null()) {
            unsafe {
                (*temp).value = value;
            }
            return true;
        }

        return false;
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

            unsafe {
                (*new_node).next = (*temp).next;
                (*temp).next = new_node;
            }

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
            let prev = self.get(index - 1);
            unsafe {
                let temp = (*prev).next;
                (*prev).next = (*temp).next;
            }

            self.length -= 1;
        }
    }

    pub fn reverse(&mut self) {
        let mut temp = self.head;
        self.head = self.tail;
        self.tail = temp;
        let mut before = ptr::null_mut();

        unsafe {
            for _i in 0..self.length {
                let after = (*temp).next;
                (*temp).next = before;
                before = temp;
                temp = after;
            }
        }
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

    println!("delete last");
    my_linked_list.delete_last();
    my_linked_list.print_list();
    my_linked_list.get_tail();

    println!("{:?}", my_linked_list);

    let get_node_0 = my_linked_list.get(0);
    let get_node_1 = my_linked_list.get(1);
    let get_node_2 = my_linked_list.get(2);
    let get_node_8 = my_linked_list.get(8);
    println!(
        "node 0: {:?} \n node 1: {:?} \n node 2: {:?} \n node 8: {:?}",
        get_node_0, get_node_1, get_node_2, get_node_8
    );

    println!("changed {}", my_linked_list.set(4, 2));

    println!("prepend list");
    my_linked_list.prepend(9);
    my_linked_list.get_head();
    my_linked_list.print_list();

    println!("new functionality");
    my_linked_list.insert(2, 10);
    my_linked_list.print_list();
    println!("here work");
    my_linked_list.delete_node(3);
    my_linked_list.print_list();
    my_linked_list.reverse();
    my_linked_list.print_list();

    println!("delete first");
    my_linked_list.delete_first();
    my_linked_list.print_list();

    println!("deleting all node");
    my_linked_list.delete_last();
    my_linked_list.delete_last();
    my_linked_list.delete_last();
    println!("{:?}", my_linked_list);
}
