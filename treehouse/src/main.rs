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
    let mut visitor_list = vec![
        Visitor::new("todd", "Hello Todd, enjoy your treehouse."),
        Visitor::new("patty", "Hi Patty. Your wine is in the fridge."),
        Visitor::new("lady girl", "Wow, who invited Lady Girl?"),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        if let Some(visitor) = known_visitor {
            visitor.greet_visitor();
        } else {
            if name.is_empty() {
                break;
            }
            println!("{} is not on the visitor list.", name);
            visitor_list.push(Visitor::new(&name, "New friend"));
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
