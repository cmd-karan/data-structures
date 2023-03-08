use std::fmt::Debug;

use crate::stack::Stack;

#[derive(Debug)]
struct Queue<T> {
    stack1: Stack<T>,
    stack2: Stack<T>,
}

impl<T: Debug> Queue<T> {
    fn new() -> Self {
        Self {
            stack1: Stack::<T>::new(),
            stack2: Stack::<T>::new()
        }
    }
    fn print(&self) {
        println!("{:#?}", self);
    }
    fn push(&mut self, item: T) {
        self.stack2.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        if self.stack1.top == -1 {
            while self.stack2.top != -1 {
                let item = self.stack2.pop().unwrap();
                self.stack1.push(item);
            }
        }
        if self.stack1.top == -1 {
            println!("Queue underflow...");
            return None;
        }
        self.stack1.pop()
    }
}

pub fn driver() {
    let mut queue = Queue::<i32>::new();
    queue.print();
    queue.pop();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.print();
    queue.pop();
    queue.print();
}