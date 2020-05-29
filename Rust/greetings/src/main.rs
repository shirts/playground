struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn greet(&self, other_name: String) -> String {
        let greeting = format!("Hello {}, my name is {}", other_name, self.name);
        println!("{}", greeting);
        greeting
    }
}

fn main() {
    let kevin = Person {
        name: String::from("Kevin"),
        age: 21
    };

    kevin.greet(String::from("Bob"));
}
