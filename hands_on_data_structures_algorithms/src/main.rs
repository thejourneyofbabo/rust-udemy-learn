use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Faild to reada line");

    let input = input.trim();

    let lett: String = input
        .split('-')
        .map(|word| word.chars().next().unwrap())
        .collect();

    println!("{}", lett);
}
