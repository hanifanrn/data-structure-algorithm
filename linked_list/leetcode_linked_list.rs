use std::ptr;

struct Node {
    val: i32,
    next: *mut Node,
}

impl Node {
    pub fn new(val: i32) -> *mut Self {
        Box::into_raw(Box::new(Node {
            val,
            next: ptr::null_mut(),
        }))
    }
}

struct MyLinkedList {
    head: *mut Node,
    tail: *mut Node,
    length: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        return MyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            length: 0,
        };
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.length as i32 {
            return -1;
        } else {
            let mut temp = self.head;

            for _i in 0..index {
                unsafe {
                    temp = (*temp).next;
                }
            }

            unsafe {
                return (*temp).val;
            }
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Node::new(val);

        if self.length == 0 {
            self.head = new_node;
            self.tail = new_node;
            self.length += 1;
        } else {
            unsafe {
                (*new_node).next = self.head;
            }
            self.head = new_node;
            self.length += 1;
        }
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Node::new(val);

        if self.length == 0 {
            self.head = new_node;
            self.tail = new_node;
            self.length += 1;
        } else {
            unsafe {
                (*self.tail).next = new_node;
            }
            self.tail = new_node;
            self.length += 1;
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.length as i32 {
            return;
        } else if index == 0 {
            self.add_at_head(val);
        } else if index == self.length as i32 {
            self.add_at_tail(val);
        } else {
            let new_node = Node::new(val);
            let mut temp = self.head;

            for _i in 0..index - 1 {
                unsafe {
                    temp = (*temp).next;
                }
            }

            unsafe {
                (*new_node).next = (*temp).next;
                (*temp).next = new_node;
            }

            self.length += 1;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.length as i32 || self.length == 0 {
            return;
        } else if self.length == 1 {
            self.head == ptr::null_mut();
            self.tail == ptr::null_mut();
            self.length -= 1;
        } else if index == 0 {
            unsafe {
                self.head = (*self.head).next;
            }

            self.length -= 1;
        } else if index == self.length as i32 - 1 {
            let mut temp = self.head;
            let mut prev = self.head;

            unsafe {
                while !((*temp).next.is_null()) {
                    prev = temp;
                    temp = (*temp).next;
                }
            }

            self.tail = prev;
            unsafe {
                (*self.tail).next = ptr::null_mut();
            }

            self.length -= 1;
        } else {
            let mut prev = self.head;

            for _i in 0..index - 1 {
                unsafe { prev = (*prev).next }
            }
            unsafe {
                let temp = (*prev).next;
                (*prev).next = (*temp).next;
            }

            self.length -= 1;
        }
    }
}
