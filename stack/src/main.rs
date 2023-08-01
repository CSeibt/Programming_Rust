struct Stack<T> {
 data: Vec<T>,
}

impl<T> Stack<T>{
    fn new() -> Stack<T> {
        Stack{
            data: Vec::new(),
        }
    }
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

impl<T: std::fmt::Display> Print for Stack<T> {
    fn print(&self) {
        for item in &self.data {
            println!("{}, ", item);
        }
    }
}

trait Print {
    fn print(&self) {}
    
}
fn main() {
    let mut first_stack = Stack::new();
    first_stack.push(2);
    first_stack.push(-3);
    first_stack.push(44);
    first_stack.push(-555);
    let a = match first_stack.pop() {
        Some(content) => content,
        None => 0
    };

    println!("{}", a);
    first_stack.print();
    first_stack.pop();
    first_stack.pop();
    first_stack.pop();
    first_stack.pop();
    first_stack.pop();
    let peek_first = first_stack.peek();
    match peek_first {
        Some(top) => println!("{}", top),
        None => println!("None")
    }

    let mut second_stack = Stack::new();
    second_stack.push(String::from("Hello"));
    second_stack.push(String::from("World"));
    
    second_stack.print();
    
}
