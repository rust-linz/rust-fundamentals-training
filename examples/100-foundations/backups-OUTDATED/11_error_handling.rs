use std::{
    fs::File,
    io::{self, Read},
};

/// Steps:
/// 1. Make param optional
/// 2. Match vs unwrap
/// 3. Bubble up

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

fn main() {
    match read_username_from_file("username.txt") {
        Ok(result) => println!("Username is {}", result),
        Err(err) => println!("Error occured! {}", err),
    }
}
