use std::{
    fs::File,
    io::{self, Read},
};

use futures::{FutureExt, join, select};

use async_std::task;

async fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

async fn do_something_wrong() {
    let res_one = read_file("username.txt").await;
    let res_two = read_file("stairway.md").await;
    println!("{}", res_one.unwrap());
    println!("{}", res_two.unwrap());
}


async fn do_something() {
    let one = read_file("username.txt");
    let two = read_file("stairway.md");
    let (res_one, res_two) = join!(one, two);
    println!("{}", res_one.unwrap());
    println!("{}", res_two.unwrap());
}


fn main() {
    task::block_on(do_something())
}

