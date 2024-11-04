#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    pub fn print(&self) {}
}

fn main() {
    println!("Hello,");
}
