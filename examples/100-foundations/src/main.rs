trait Greeter {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greeter for Person {
    fn greet(&self) -> String {
        format!("Hey {}!", self.name)
    }
}

struct Dog;

impl Greeter for Dog {
    fn greet(&self) -> String {
        "Who is a good boy?".to_string()
    }
}

fn get_a_greeter(val: u8) -> Box<dyn Greeter> {
    if val < 5 {
        Box::new(Person {
            name: "unknown".to_string(),
        })
    } else {
        Box::new(Dog {})
    }
}

fn let_the_greeter_greet(greeter: &dyn Greeter) {
    println!("{}", greeter.greet());
}

fn main() {
    let greeter = get_a_greeter(4);
    let_the_greeter_greet(greeter.as_ref());
}
