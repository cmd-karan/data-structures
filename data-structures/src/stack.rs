#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
    top: i32
}

impl<T: std::fmt::Debug> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            stack: Vec::new(),
            top: -1
        }
    }
    fn print(&self) {
        println!("{:#?}", self);
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
        self.top += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.top >=0 {
            self.stack.pop()
        } else {
            println!("Stack underflow...");
            None
        }
    }

}

pub fn driver() {
    let mut stack = Stack::<i32>::new();
    stack.print();
    stack.pop();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.print();
    stack.pop();
    stack.print();
}