#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name  = {}, age = {} has {} children",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "matt".to_string(),
        age: 35,
        children: 4,
        fave_color: Color::Green,
    };

    let c = Color::Red("Hello".to_string());

    match c {
        Color::Red(s) => println!("It's red {}", s),
        Color::Green => println!("It's Green"),
        Color::Blue => println!("It's Blue"),
    }

    //println!("Hello, people, from {:?}", p.print());
    println!("Hello, people, from {:?}", p);
}
