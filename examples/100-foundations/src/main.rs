use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

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

impl Default for Person {
    fn default() -> Self {
        Person {
            name: "unknown".to_string(),
        }
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

fn parse_number_from_file(path: String) -> Result<u32, Box<dyn Error>> {
    let mut handle = std::fs::File::open(path)?;
    let mut buf = String::new();
    handle.read_to_string(&mut buf)?;
    let number: u32 = buf.trim().parse()?;
    Ok(number)
}

fn let_the_greeter_greet(greeter: &dyn Greeter) {
    println!("{}", greeter.greet());
}

fn main() {
    let person: Person = Default::default();
    println!("{}", person.greet());
    let greeter = get_a_greeter(4);
    let_the_greeter_greet(greeter.as_ref());

    match parse_number_from_file("number.txt".to_string()) {
        Ok(num) => println!("{}", num),
        Err(err) => println!("{}", err),
    };

    let some_vec = vec![0, 1, 2, 3];
    let mut iter = some_vec.iter();
    println!("{}", iter.nth(0).unwrap());
    let first_0 = some_vec.get(0);
    let second_0 = some_vec.get(0);
    println!("{}", first_0.unwrap());
    println!("{}", second_0.unwrap());

    let f = File::open("stairway.md").unwrap();
    let mut lines = BufReader::new(f).lines();

    for line in lines {
        let line = line.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
