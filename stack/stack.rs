// Implement Stack Using Singly Linked List
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
        return Rc::new(RefCell::new(Node { value, next: None }));
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Stack {
    top: Option<Link>,
    height: usize,
}

impl Stack {
    pub fn new(value: i32) -> Self {
        let new_node = Node::new(value);

        Stack {
            top: Some(new_node.clone()),
            height: 1,
        }
    }

    pub fn get_top(&self) {
        if self.top.is_some() {
            println!("Top: {}", self.top.as_ref().unwrap().borrow().value);
        } else {
            println!("Top: None");
        }
    }

    pub fn get_height(&self) {
        println!("Height: {}", self.height);
    }

    pub fn print_stack(&self) {
        let mut temp = self.top.clone();
        while temp.is_some() {
            println!("{}", temp.as_ref().unwrap().borrow().value);
            temp = temp.clone().unwrap().borrow().next.clone();
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.height == 0 {
            return true;
        }

        return false;
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Node::new(value);
        new_node.borrow_mut().next = self.top.clone();
        self.top = Some(new_node.clone());
        self.height += 1;
    }

    pub fn top_value(&self) -> Result<i32, &'static str> {
        match self.height {
            0 => Err("the stack is empty"),
            _ => Ok(self.top.as_ref().unwrap().borrow().value),
        }
    }

    pub fn pop(&mut self) -> Result<i32, &'static str> {
        match self.height {
            0 => Err("the stack is empty"),
            _ => {
                let popped_value = self.top.as_ref().unwrap().borrow().value;
                self.top = self.top.clone().as_ref().unwrap().borrow().next.clone();
                self.height -= 1;
                return Ok(popped_value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Stack;

    #[test]
    fn is_empty_test() {
        let my_stack = Stack::new(0);
        assert_eq!(false, my_stack.is_empty());
    }

    #[test]
    fn push_test() {
        let mut my_stack = Stack::new(0);
        assert_eq!(0, my_stack.top_value().unwrap());
        my_stack.push(1);
        assert_eq!(1, my_stack.top_value().unwrap());
        my_stack.push(2);
        assert_eq!(2, my_stack.top_value().unwrap());
    }

    #[test]
    fn pop_test() {
        let mut my_stack = Stack::new(0);
        assert_eq!(0, my_stack.pop().unwrap());
        my_stack.push(0);
        my_stack.push(1);
        my_stack.push(2);
        assert_eq!(2, my_stack.pop().unwrap());
        assert_eq!(1, my_stack.pop().unwrap());
    }
}

fn main() {
    let mut my_stack = Stack::new(1);
    my_stack.get_top();
    my_stack.get_height();
    my_stack.print_stack();
    my_stack.is_empty();
    my_stack.push(2);
    let top_value = my_stack.top_value();
    println!("top value: {}", top_value.unwrap());
    println!("this is stack: {:#?}", my_stack);
    println!("first pop: {}", my_stack.pop().unwrap());
    println!("second pop: {}", my_stack.pop().unwrap());
    println!("third pop: {}", my_stack.pop()unwrap());
}
