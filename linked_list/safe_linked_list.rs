// using safe rust with Box smart pointer but still not fully functioning because head and tail
// cannot pointing to the same node

#[derive(Debug, PartialEq, Eq)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    length: i32,
}

impl LinkedList {
    pub fn new(value: i32) -> Self {
        return Self {
            head: Some(Box::new(Node { value, next: None })),
            tail: Some(Box::new(Node { value, next: None })),
            length: 1,
        };
    }

    pub fn print_list(&self) {
        println!("{:?}", &self)
    }

    pub fn get_head(&self) {
        if self.head == None {
            println!("Head: None");
        } else {
            println!("{:?}", self.head.as_ref().unwrap().value);
        }
    }

    pub fn get_tail(&self) {
        if self.tail == None {
            println!("Tail: None");
        } else {
            println!("{:?}", self.tail.as_ref().unwrap().value);
        }
    }

    pub fn get_length(&self) {
        println!("Length: {}", self.length);
    }

    fn get_node_at_index(&mut self, index: i32) -> &mut Option<Box<Node>> {
        let mut cur = &mut self.head;

        for _ in 0..index {
            if cur.is_none() {
                return cur;
            }

            cur = &mut cur.as_mut().unwrap().next;
        }

        return cur;
    }

    fn add_at_index(&mut self, index: i32, value: i32) {
        let cur = self.get_node_at_index(index - 1);

        if let Some(ref mut cur) = cur {
            let new_node = Some(Box::new(Node {
                value,
                next: cur.next.take(),
            }));

            cur.next = new_node;
            self.length += 1;
        }
    }

    pub fn append(&mut self, value: i32) {
        let tail = self.tail;

        match tail {
            Some(tail) => {
                let new_node = Some(Box::new(Node {
                    value,
                    next: cur.next.take(),
                }));

                cur.next = new_node;
                self.length += 1;
            }
            _ => {}
        }

        self.add_at_index(self.length, value);
    }
}

fn main() {
    let mut my_linked_list = LinkedList::new(1);

    my_linked_list.get_head();
    my_linked_list.get_tail();
    my_linked_list.get_length();

    println!("Linked List:");
    my_linked_list.print_list();

    my_linked_list.append(2);
    my_linked_list.print_list();

    my_linked_list.append(3);
    my_linked_list.print_list();
    /*
     * EXPECTED OUTPUT:
     * Head: 4
     * Tail: 4
     * Length: 1
     *
     * Linked List:
     * 4
     */
}
