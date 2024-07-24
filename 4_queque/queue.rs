use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    value: i32,
    next: Option<Link>,
}

impl Node {
    fn new(value: i32) -> Link {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Queue {
    first: Option<Link>,
    last: Option<Link>,
    length: usize,
}

impl Queue {
    pub fn new(value: i32) -> Queue {
        let new_node = Node::new(value);
        Queue {
            first: Some(new_node.clone()),
            last: Some(new_node.clone()),
            length: 1,
        }
    }

    pub fn print_queue(&self) {
        let mut temp = self.first.clone();
        while temp.is_some() {
            println!("{}", temp.as_ref().unwrap().borrow().value);
            temp = temp.clone().unwrap().borrow().next.clone();
        }
    }

    pub fn get_first(&self) {
        if self.first == None {
            println!("First: None");
        } else {
            println!("First: {}", self.first.as_ref().unwrap().borrow().value);
        }
    }

    pub fn get_last(&self) {
        if self.last == None {
            println!("Last: None");
        } else {
            println!("Last: {}", self.last.as_ref().unwrap().borrow().value);
        }
    }

    pub fn get_length(&self) {
        println!("Length: {}", self.length);
    }

    pub fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true;
        }
        return false;
    }

    pub fn first_value(&self) -> Result<i32, &'static str> {
        match &self.first {
            Some(node) => Ok(node.borrow().value),
            None => Err("the queue is empty"),
        }
    }

    pub fn last_value(&self) -> Result<i32, &'static str> {
        match &self.last {
            Some(node) => Ok(node.borrow().value),
            None => Err("the queue is empty"),
        }
    }

    pub fn enqueue(&mut self, value: i32) {
        let new_node = Node::new(value);
        if self.length == 0 {
            self.first = Some(new_node.clone());
            self.last = Some(new_node.clone());
        } else {
            self.last.as_mut().unwrap().borrow_mut().next = Some(new_node.clone());
            self.last = Some(new_node.clone());
        }

        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Result<i32, &'static str> {
        match self.length {
            0 => Err("the queue is empty"),
            1 => {
                let dequeue_value = self.first.as_ref().unwrap().borrow().value;
                self.first = None;
                self.last = None;
                self.length -= 1;
                return Ok(dequeue_value);
            }
            _ => {
                let dequeue_value = self.first.as_ref().unwrap().borrow().value;
                let temp = self.first.clone();
                self.first = temp.as_ref().unwrap().borrow().next.clone();
                self.length -= 1;
                return Ok(dequeue_value);
            }
        }
    }
}

fn main() {
    let my_queue = Queue::new(1);
    println!("{:#?}", my_queue);
    my_queue.print_queue();
    my_queue.get_first();
    my_queue.get_last();
    let is_empty = my_queue.is_empty();
    println!("{}", is_empty);
    let first_value = my_queue.first_value().unwrap();
    println!("{}", first_value);
    my_queue.get_length();
}

#[cfg(test)]
mod test {
    use crate::Queue;

    #[test]
    fn new_stack_test() {
        let my_queue = Queue::new(1);
        assert_eq!(my_queue.first_value().unwrap(), 1);
        assert_eq!(my_queue.last_value().unwrap(), 1);
    }

    #[test]
    fn enqueue_test() {
        let mut my_queue = Queue::new(1);
        assert_eq!(my_queue.length, 1);
        my_queue.enqueue(2);
        assert_eq!(my_queue.first_value().unwrap(), 1);
        assert_eq!(my_queue.last_value().unwrap(), 2);
        assert_eq!(my_queue.length, 2);
        my_queue.enqueue(3);
        assert_eq!(my_queue.last_value().unwrap(), 3);
        assert_eq!(my_queue.length, 3);
    }

    #[test]
    fn dequeue_test() {
        let mut my_queue = Queue::new(1);
        assert_eq!(my_queue.length, 1);
        assert_eq!(Ok(1), my_queue.dequeue());
        assert_eq!(0, my_queue.length);
        assert_eq!(Err("the queue is empty"), my_queue.dequeue());
        assert_eq!(Err("the queue is empty"), my_queue.first_value());
        assert_eq!(Err("the queue is empty"), my_queue.last_value());
        my_queue.enqueue(1);
        my_queue.enqueue(2);
        my_queue.enqueue(3);
        assert_eq!(Ok(1), my_queue.dequeue());
        assert_eq!(Ok(2), my_queue.dequeue());
    }
}
