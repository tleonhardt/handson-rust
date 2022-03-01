#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("todd", "Hello Todd, enjoy your treehouse."),
        Visitor::new("patty", "Hi Patty. Your wine is in the fridge."),
        Visitor::new("lady girl", "Wow, who invited Lady Girl?"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}
